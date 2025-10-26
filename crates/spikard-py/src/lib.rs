//! Python bindings for spikard
//!
//! This crate provides Python bindings using PyO3

use pyo3::prelude::*;

/// Process using spikard
#[pyfunction]
fn process() -> PyResult<()> {
    spikard::process().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Spikard error: {}", e))
    })
}

/// Python module for spikard
#[pymodule]
fn _spikard(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
