use std::{collections::BTreeMap, path::PathBuf};

use wasmtime::Module;

const FUNCTIONS: [&str; 5] = ["hello", "add", "sub", "mul", "div"];

// TODO: in theory this would load from a set of addresses based on a list of assigned functions (depending on capacity etc)
pub async fn load_functions(
    function_dir: &PathBuf,
    engine: &wasmtime::Engine,
) -> anyhow::Result<BTreeMap<String, Module>> {
    let mut function_map = BTreeMap::new();
    let function_dir = function_dir.canonicalize()?.display().to_string();

    for fn_name in FUNCTIONS {
        let template_string =
            format!("{function_dir}/{fn_name}/target/wasm32-wasip1/debug/{fn_name}.wasm");
        let module = Module::from_file(engine, template_string)?;
        function_map.insert(fn_name.to_string(), module);
    }
    Ok(function_map)
}
