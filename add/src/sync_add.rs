use std::path::PathBuf;

use anyhow::Context;
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};

use crate::state::States;

mod bindings {
    wasmtime::component::bindgen!({
        path: "../wit/adder/world.wit",
        world: "adder",
    });
}

pub fn add(path: PathBuf, x: u32, y: u32) -> wasmtime::Result<u32> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, path)?;

    let wasi_view = States::new();
    let mut store = Store::new(&engine, wasi_view);

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).expect("Could not add wasi to linker");

    let instance = bindings::Adder::instantiate(&mut store, &component, &linker)?;
    instance
        .docs_adder_add()
        .call_add(&mut store, x, y)
        .context("calling add function")
}
