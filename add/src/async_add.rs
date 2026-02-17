use std::path::PathBuf;

use anyhow::Context;
use wasmtime::{
    Config, Store,
    component::{Component, Linker},
};

use crate::state::States;

mod bindings {
    wasmtime::component::bindgen!({
        path: "../wit/adder/world.wit",
        world: "adder",
        imports: { default: async },
        exports: { default: async },
    });
}

pub async fn add(path: PathBuf, x: u32, y: u32) -> wasmtime::Result<u32> {
    let mut config = Config::default();
    config.async_support(true);
    let engine = wasmtime::Engine::new(&config)?;

    let component = Component::from_file(&engine, path).context("failed to load component")?;

    let wasi_view = States::new();
    let mut store = Store::new(&engine, wasi_view);

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    let instance = bindings::Adder::instantiate_async(&mut store, &component, &linker)
        .await
        .context("failed to instantiate component")?;
    instance
        .docs_adder_add()
        .call_add(&mut store, x, y)
        .await
        .context("failed to call add function")
}
