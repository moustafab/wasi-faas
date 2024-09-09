use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use api::worker::{Worker, WorkerId, WorkerStatus};
use axum::{routing::post, Router};
use clap::Parser;
use function::load_functions;
use reqwest::Client;
use serde_json::json;
use wasmtime::{Config, Engine, Store};

mod executor;
mod function;

#[derive(clap::Parser)]
struct Args {
    #[clap(long, default_value = "http://localhost:3000")]
    control_plane_address: String,
    #[clap(long, default_value = "127.0.0.1:3001")]
    address: String,
    #[clap(long, default_value = "data/worker_id")]
    worker_id_file: PathBuf,
    #[clap(long, default_value = "functions-sample/")]
    function_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();
    let Args {
        control_plane_address,
        worker_id_file,
        address,
        function_dir,
    } = Args::try_parse()?;
    let worker_id =
        match register_worker(&client, &worker_id_file, &control_plane_address, &address).await {
            Ok(worker_id) => worker_id,
            Err(e) => {
                tracing::error!("worker not found");
                tracing::warn!(
                    "Clearing out the {} file to reset",
                    worker_id_file.clone().display()
                );
                std::fs::remove_file(worker_id_file.clone())?;
                register_worker(&client, &worker_id_file, &control_plane_address, &address).await?
            }
        };

    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let function_map = Arc::new(load_functions(&function_dir, &engine).await?);

    let function_executor_api = Router::new()
        .route("/hello", post(executor::execute_hello))
        .with_state((function_map, engine));
    // .route("/add", post(functions::execute_add))
    // .route("/sub", post(functions::execute_sub))
    // .route("/mul", post(functions::execute_mul));

    let app = Router::new().nest("/execute", function_executor_api);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    tokio::select! {
        _ = heartbeat_loop(client.clone(), control_plane_address.clone(), worker_id) => {
            tracing::info!("shutting down, heartbeat loop ended");
        },
        _ = axum::serve(listener, app) => {
            tracing::info!("shutting down, server ended");
        }
    }
    Ok(())
}

async fn heartbeat_loop(
    client: Client,
    control_plane_address: String,
    worker_id: WorkerId,
) -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        heartbeat(&client, &control_plane_address, &worker_id).await?;
    }
}

async fn get_or_create_worker_id(
    client: &Client,
    worker_id_file: &PathBuf,
    control_plane_address: &str,
    address: &str,
) -> anyhow::Result<WorkerId> {
    match std::fs::read_to_string(worker_id_file.clone()) {
        Ok(worker_id) => Ok(WorkerId::parse(&worker_id)?),
        Err(_) => {
            // create a new worker with the api
            let worker = client
                .post(format!("{control_plane_address}/workers"))
                .json(&address)
                .send()
                .await?
                .json::<Worker>()
                .await?;
            std::fs::write(worker_id_file.clone(), worker.id().to_string())?;
            Ok(*worker.id())
        }
    }
}

/// Register the worker with the control plane and return the worker
async fn register_worker(
    client: &Client,
    worker_id_file: &PathBuf,
    control_plane_address: &str,
    address: &str,
) -> anyhow::Result<WorkerId> {
    let worker_id =
        get_or_create_worker_id(&client, &worker_id_file, &control_plane_address, &address).await?;

    let worker_url = format!("{control_plane_address}/workers/{}", worker_id);

    // update the worker status to available and the address to the current address
    let current_worker: Option<Worker> =
        client.get(worker_url.clone()).send().await?.json().await?;

    tracing::info!("worker status: {:?}", current_worker);

    if let Some(mut worker) = current_worker {
        // set the worker status to available and update the address
        worker.update_address(address.to_string().into());
        worker.update_status(WorkerStatus::Available);

        let worker = client
            .patch(worker_url.clone())
            .json(&json!(worker))
            .send()
            .await?
            .json::<Worker>()
            .await?;
        Ok(*worker.id())
    } else {
        Err(anyhow::anyhow!("worker not found"))
    }
}

async fn heartbeat(
    client: &Client,
    control_plane_address: &str,
    worker_id: &WorkerId,
) -> anyhow::Result<()> {
    let worker_url = format!("{control_plane_address}/workers/{}", worker_id);
    tracing::trace!("sending heartbeat to {}", worker_url);
    client
        .patch(worker_url.clone())
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
