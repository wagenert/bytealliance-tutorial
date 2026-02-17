use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

pub struct States {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl States {
    pub fn new() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().build(),
        }
    }
}

impl WasiView for States {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView {
            table: &mut self.table,
            ctx: &mut self.ctx,
        }
    }
}
