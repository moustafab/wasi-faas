use std::{collections::BTreeMap, sync::Arc};

use axum::{extract::State, response::IntoResponse};
use wasmtime::{Linker, Module, Store};
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

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |cx| cx).unwrap();

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build_p1();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    linker.module(&mut store, "", module).unwrap();
    let instance = linker.instantiate_async(&mut store, &module).await.unwrap();
    let exports = instance.module(&mut store).exports();
    for export in exports {
        tracing::info!("Found export \"{:?}\"", export);
    }
    instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .unwrap()
        .call_async(&mut store, ())
        .await
        .unwrap();
    "Hello, from rust code!"
}

#[tracing::instrument(level = "info")]
pub async fn execute_add() -> impl IntoResponse {
    tracing::info!("executing");
    todo!("implement me");
}

#[tracing::instrument(level = "info")]
pub async fn execute_sub() -> impl IntoResponse {
    tracing::info!("executing");
    todo!("implement me");
}

#[tracing::instrument(level = "info")]
pub async fn execute_mul() -> impl IntoResponse {
    tracing::info!("executing");
    todo!("implement me");
}
