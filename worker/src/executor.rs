use std::{collections::BTreeMap, sync::Arc};

use axum::{extract::State, response::IntoResponse, Json};
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

#[tracing::instrument(level = "info", skip(function_map, engine))]
pub async fn execute_hello(
    State((function_map, engine)): State<(Arc<BTreeMap<String, Module>>, wasmtime::Engine)>,
) -> impl IntoResponse {
    tracing::info!("executing");

    let module = function_map
        .get("hello")
        .ok_or_else(|| anyhow::anyhow!("Couldn't load the hello function"))
        .unwrap();
    execute_wasm::<(), ()>(module, engine, "_start", ())
        .await
        .unwrap();
}

#[tracing::instrument(level = "info", skip(function_map, engine))]
pub async fn execute_add(
    State((function_map, engine)): State<(Arc<BTreeMap<String, Module>>, wasmtime::Engine)>,
    Json((param1, param2)): Json<(i32, i32)>,
) -> impl IntoResponse {
    tracing::info!("executing");
    let module = function_map
        .get("add")
        .ok_or_else(|| anyhow::anyhow!("Couldn't load the add function"))
        .unwrap();
    let result = execute_wasm::<(i32, i32), i32>(module, engine, "add", (param1, param2))
        .await
        .unwrap();
    Json(result)
}

#[tracing::instrument(level = "info", skip(function_map, engine))]
pub async fn execute_sub(
    State((function_map, engine)): State<(Arc<BTreeMap<String, Module>>, wasmtime::Engine)>,
    Json((param1, param2)): Json<(i32, i32)>,
) -> impl IntoResponse {
    tracing::info!("executing");
    let module = function_map
        .get("sub")
        .ok_or_else(|| anyhow::anyhow!("Couldn't load the sub function"))
        .unwrap();
    let result = execute_wasm::<(i32, i32), i32>(module, engine, "sub", (param1, param2))
        .await
        .unwrap();
    Json(result)
}

#[tracing::instrument(level = "info", skip(function_map, engine))]
pub async fn execute_mul(
    State((function_map, engine)): State<(Arc<BTreeMap<String, Module>>, wasmtime::Engine)>,
    Json((param1, param2)): Json<(i32, i32)>,
) -> impl IntoResponse {
    tracing::info!("executing");
    let module = function_map
        .get("mul")
        .ok_or_else(|| anyhow::anyhow!("Couldn't load the mul function"))
        .unwrap();
    let result = execute_wasm::<(i32, i32), i32>(module, engine, "mul", (param1, param2))
        .await
        .unwrap();
    Json(result)
}

#[tracing::instrument(level = "info", skip(function_map, engine))]
pub async fn execute_div(
    State((function_map, engine)): State<(Arc<BTreeMap<String, Module>>, wasmtime::Engine)>,
    Json((param1, param2)): Json<(i32, i32)>,
) -> impl IntoResponse {
    tracing::info!("executing");
    let module = function_map
        .get("div")
        .ok_or_else(|| anyhow::anyhow!("Couldn't load the div function"))
        .unwrap();
    let result = execute_wasm::<(i32, i32), i32>(module, engine, "div", (param1, param2))
        .await
        .unwrap();
    Json(result)
}

async fn execute_wasm<Input: wasmtime::WasmParams, Output: wasmtime::WasmResults>(
    module: &Module,
    engine: Engine,
    entrypoint_name: &str,
    params: Input,
) -> anyhow::Result<Output> {
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |cx| cx)?;

    // TODO: bind stdout to a buffer and return the buffer as the response instead
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build_p1();
    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate_async(&mut store, module).await?;
    instance
        .get_typed_func::<Input, Output>(&mut store, entrypoint_name)
        .map_err(|err| anyhow::anyhow!("Couldn't find the entrypoint: {}", err))?
        .call_async(&mut store, params)
        .await
}
