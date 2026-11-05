#![allow(dead_code)]
// Wasmtime-backed sandbox for dynamic WebAssembly AST patch components.

use std::fs;
use std::path::Path;
use wasmtime::{Config, Engine, Instance, Module, Store};

/// A resource-bounded WebAssembly runtime used to execute AST patch components.
pub struct WasmRuntime {
    engine: Engine,
}

impl WasmRuntime {
    /// Create a new runtime with fuel-based execution limits.
    pub fn new() -> Result<Self, String> {
        let mut config = Config::default();
        config.consume_fuel(true);
        config.max_wasm_stack(512 * 1024);

        let engine = Engine::new(&config).map_err(|e| e.to_string())?;
        Ok(Self { engine })
    }

    /// Load, instantiate, and execute a WebAssembly patch component if present.
    ///
    /// The component is expected to export either `_start` or `run`. Execution
    /// is bounded by a fixed fuel budget to keep the sandbox resource-limited.
    pub fn execute_patch(&self, path: &str) -> Result<(), String> {
        if !Path::new(path).exists() {
            return Err(format!("Wasm patch not found: {}", path));
        }

        let bytes = fs::read(path).map_err(|e| e.to_string())?;
        let module = Module::new(&self.engine, &bytes).map_err(|e| e.to_string())?;

        let mut store = Store::new(&self.engine, ());
        store
            .set_fuel(100_000)
            .map_err(|e| e.to_string())?;

        let instance = Instance::new(&mut store, &module, &[]).map_err(|e| e.to_string())?;

        if let Ok(func) = instance.get_typed_func::<(), ()>(&mut store, "_start") {
            func.call(&mut store, ()).map_err(|e| e.to_string())?;
        } else if let Ok(func) = instance.get_typed_func::<(), ()>(&mut store, "run") {
            func.call(&mut store, ()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
