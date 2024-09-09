use axum::{
    routing::{get, post},
    Router,
};
use workers::WorkerStore;

mod api_gateway;
mod workers;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let worker_store = WorkerStore::new();
    let workers_api = Router::new()
        .route("/", get(workers::list_workers).post(workers::create_worker))
        .route(
            "/:id",
            get(workers::get_worker)
                .patch(workers::update_worker)
                .delete(workers::delete_worker),
        )
        .with_state(worker_store.clone());

    // proxy calls to the first available worker in api-gateway
    let api_gateway = Router::new()
        .route("/:function", post(api_gateway::proxy))
        .with_state(worker_store);

    // TODO: in theory we'd be able to have an api for registering functions and their associated paths but for simplicity we'll just hardcode them
    let app = Router::new()
        .nest("/workers", workers_api)
        .nest("/api", api_gateway);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}
