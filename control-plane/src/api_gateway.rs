use api::{types::JsonData, worker::WorkerStatus};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::workers::WorkerStore;

#[tracing::instrument(skip(store))]
pub async fn proxy(
    State(store): State<WorkerStore>,
    Path(function): Path<String>,
    Json(payload): Json<JsonData>,
) -> Result<Json<JsonData>, StatusCode> {
    let workers = store.list(Some(&WorkerStatus::Available));
    if workers.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    let worker = workers.iter().next().unwrap();
    tracing::info!("proxying request to worker: {}", worker.id());
    let url = format!("http://{}/execute/{}", worker.address(), function);
    let client = reqwest::Client::new();
    let response = client.post(&url).json(&payload).send().await;
    match response {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                let body = response.json().await.map_err(|e| {
                    tracing::error!(error = ?e, "failed to parse response body");
                    StatusCode::BAD_GATEWAY
                })?;
                Ok(Json(body))
            } else {
                Err(status)
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}
