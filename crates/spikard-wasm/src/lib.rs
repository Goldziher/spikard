//! WebAssembly bindings for spikard
//!
//! This crate provides WebAssembly bindings using wasm-bindgen

use wasm_bindgen::prelude::*;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Placeholder entry-point for the WASM bindings.
#[wasm_bindgen]
pub fn process() -> Result<(), JsValue> {
    Err(JsValue::from_str(
        "Spikard WASM bindings are not ready for the new App API",
    ))
}
