use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, Query, State},
    Json,
};

use api::worker::{Worker, WorkerAddress, WorkerId, WorkerStatus};
use tracing::instrument;

#[derive(Clone)]
pub struct WorkerStore {
    inner: Arc<Mutex<BTreeMap<WorkerId, Worker>>>,
}

impl WorkerStore {
    pub fn new() -> Self {
        WorkerStore {
            inner: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
    pub fn insert(&mut self, worker: Worker) {
        self.inner.lock().unwrap().insert(*worker.id(), worker);
    }
    pub fn list(&self, status: Option<&WorkerStatus>) -> Vec<Worker> {
        self.inner
            .lock()
            .unwrap()
            .values()
            .filter(|w| {
                if let Some(status) = status {
                    w.status() == status
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
    pub fn get(&self, id: WorkerId) -> Option<Worker> {
        self.inner.lock().unwrap().get(&id).cloned()
    }
    pub fn update(&mut self, worker: &Worker) -> Option<Worker> {
        if let Some(entry) = self.inner.lock().unwrap().get_mut(worker.id()) {
            *entry = worker.clone();
            Some(worker.clone())
        } else {
            None
        }
    }
    pub fn touch(&mut self, id: &WorkerId) -> Option<Worker> {
        if let Some(entry) = self.inner.lock().unwrap().get_mut(id) {
            entry.touch();
            Some(entry.clone())
        } else {
            None
        }
    }
    pub fn remove(&mut self, id: &WorkerId) -> Option<Worker> {
        self.inner.lock().unwrap().remove(id)
    }
}

// FIXME: fix error handling to return 404 when worker not found

#[tracing::instrument(skip(store))]
pub async fn list_workers(
    State(store): State<WorkerStore>,
    maybe_status: Option<Query<WorkerStatus>>,
) -> Json<Vec<Worker>> {
    Json(store.list(maybe_status.map(|Query(status)| status).as_ref()))
}

#[tracing::instrument(skip(store))]
pub async fn create_worker(
    State(mut store): State<WorkerStore>,
    Json(address): Json<WorkerAddress>,
) -> Json<Worker> {
    let worker = Worker::new(address);
    store.insert(worker.clone());
    Json(worker)
}

#[tracing::instrument(skip(store))]
pub async fn get_worker(
    State(store): State<WorkerStore>,
    Path(worker_id): Path<WorkerId>,
) -> Json<Option<Worker>> {
    Json(store.get(worker_id))
}

#[tracing::instrument(skip(store))]
pub async fn update_worker(
    State(mut store): State<WorkerStore>,
    Path(worker_id): Path<WorkerId>,
    maybe_worker_json: Option<Json<Worker>>,
) -> Json<Option<Worker>> {
    if let Some(Json(worker)) = maybe_worker_json {
        let result = store.update(&worker);
        Json(result)
    } else {
        let result = store.touch(&worker_id);
        Json(result)
    }
}

// FIXME: don't actually delete the worker, just mark it as deleted and disabled
pub async fn delete_worker(
    State(mut store): State<WorkerStore>,
    Path(worker_id): Path<WorkerId>,
) -> Json<Option<Worker>> {
    Json(store.remove(&worker_id))
}
