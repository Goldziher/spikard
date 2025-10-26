//! WebAssembly bindings for spikard
//!
//! This crate provides WebAssembly bindings using wasm-bindgen

use wasm_bindgen::prelude::*;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Process using spikard
#[wasm_bindgen]
pub fn process() -> Result<(), JsValue> {
    spikard::process().map_err(|e| JsValue::from_str(&format!("Spikard error: {}", e)))
}
