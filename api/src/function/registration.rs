use derive_more::derive::Display;
use serde::{Deserialize, Serialize};

use crate::types::{BlobAddress, Id, TimeStamp};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputKind {
    None,
    List(Box<InputKind>),
    Object,
    String,
    Number,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Runtime {
    Wasm,
    // TODO: figure out how to support other runtimes
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
pub struct FunctionId(Id);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
    id: FunctionId,
    name: String,
    description: String,
    create_time: TimeStamp,
    runtime: Runtime,
    input_type: InputKind,
    blob_address: BlobAddress,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Root(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubPath(String);

#[derive(Serialize, Deserialize)]
pub struct PathEntry {
    root: Root,
    sub_path: SubPath,
    function: Function,
}

impl PathEntry {
    pub fn new(root: Root, sub_path: SubPath, function: Function) -> Self {
        PathEntry {
            root,
            sub_path,
            function,
        }
    }
    pub fn root(&self) -> &Root {
        &self.root
    }
    pub fn sub_path(&self) -> &SubPath {
        &self.sub_path
    }
    pub fn function(&self) -> &Function {
        &self.function
    }
}
