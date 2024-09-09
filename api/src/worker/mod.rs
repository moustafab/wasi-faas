use derive_more::derive::Display;
use serde::{Deserialize, Serialize};

use crate::types::{Id, TimeStamp};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WorkerStatus {
    #[default]
    Available,
    Occupied,
    Disabled,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Debug)]
pub struct WorkerId(Id);

impl WorkerId {
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(WorkerId(Id::parse(s)?))
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Display, Debug)]
pub struct WorkerAddress(String);

impl From<String> for WorkerAddress {
    fn from(s: String) -> Self {
        WorkerAddress(s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Worker {
    id: WorkerId,
    address: WorkerAddress,
    status: WorkerStatus,
    create_time: TimeStamp,
    last_heartbeat: TimeStamp,
}

impl Worker {
    pub fn new(address: WorkerAddress) -> Self {
        Worker {
            id: WorkerId(Id::new()),
            address,
            status: WorkerStatus::Available,
            create_time: TimeStamp::now(),
            last_heartbeat: TimeStamp::now(),
        }
    }
    pub fn id(&self) -> &WorkerId {
        &self.id
    }
    pub fn touch(&mut self) {
        self.last_heartbeat = TimeStamp::now();
    }
    pub fn update_address(&mut self, address: WorkerAddress) {
        self.address = address;
    }
    pub fn update_status(&mut self, status: WorkerStatus) {
        self.status = status;
    }
}
