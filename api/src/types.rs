use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Debug)]
pub struct Id(#[serde(serialize_with = "uuid::serde::simple::serialize")] Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(Id(Uuid::parse_str(s)?))
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct JsonData(Value);

impl TryFrom<Vec<u8>> for JsonData {
    type Error = serde_json::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(JsonData(serde_json::from_slice(&value)?))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ExitKind {
    Success,
    Failure { exit_code: u8 },
    TimeOut,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlobAddress(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Display, Debug)]
pub struct TimeStamp(chrono::DateTime<chrono::Utc>);

impl TimeStamp {
    pub fn now() -> Self {
        TimeStamp(chrono::Utc::now())
    }
}
