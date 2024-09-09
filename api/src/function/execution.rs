use crate::{
    types::{ExitKind, Id, JsonData, TimeStamp},
    worker::WorkerId,
};

use super::registration::FunctionId;

pub struct Input(JsonData);

pub struct Output(JsonData);

impl TryFrom<Vec<u8>> for Output {
    type Error = serde_json::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Output(JsonData::try_from(value)?))
    }
}

pub struct ExecutionRequestId(Id);

pub struct ExecutionRequest {
    id: ExecutionRequestId,
    create_time: TimeStamp,
    input: Input,
    target_function: FunctionId,
}

pub enum ExecutionStatus {
    Created,
    Assigned,
    Started,
    Completed,
    Unknown,
}

pub struct ExecutionResultId(Id);

// TODO: it'd be nice to have some way of getting logs
pub struct ExecutionResult {
    id: ExecutionResultId,
    create_time: TimeStamp,
    output_data: Option<Output>,
    exit: ExitKind,
    worker: WorkerId,
    complete_time: Option<TimeStamp>,
}

pub struct ExecutionId(Id);

pub struct Execution {
    id: ExecutionId,
    request: ExecutionRequest,
    result: Option<ExecutionResult>,
    status: ExecutionStatus,
}
