pub use app::*;
// not sure why I had to add this here but it's not on the other example:
use crux_core::Core;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;

uniffi_macros::include_scaffolding!("shared");

pub mod app;

lazy_static! {
    static ref CORE: Core<TodoMVC> = Core::new();
}

#[wasm_bindgen]
pub fn message(data: &[u8]) -> Vec<u8> {
    CORE.message(data)
}

#[wasm_bindgen]
pub fn response(data: &[u8]) -> Vec<u8> {
    CORE.response(data)
}

#[wasm_bindgen]
pub fn view() -> Vec<u8> {
    CORE.view()
}
