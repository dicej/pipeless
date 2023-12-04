component::bindgen!({
    path: "wit",
    world: "hook"
});

use {
    std::error::Error,
    wasmtime::{
        component::{self, Component, Linker},
        Config, Engine, Store,
    },
    wasmtime_wasi::preview2::{command::sync as command, Table, WasiCtx, WasiCtxBuilder, WasiView},
};

struct Ctx {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for Ctx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.debug_info(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "./process.wasm")?;
    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker)?;
    let table = Table::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, Ctx { table, wasi });
    let (hook, _) = Hook::instantiate(&mut store, &component, &linker)?;
    let result = hook.call_hook(
        &mut store,
        &Frame {
            uuid: String::new(),
            original: Vec::new(),
            modified: Vec::new(),
            width: 0,
            height: 0,
            pts: 0,
            dts: 0,
            duration: 0,
            fps: 0,
            input_timestamp: 0.0,
            inference_input: Vec::new(),
            inference_output: Vec::new(),
            pipeline_id: String::new(),
        },
        &Context {
            to_change: String::new(),
        },
    )?;
    println!("result is: {result:?}");
    Ok(())
}
