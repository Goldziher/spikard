use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::into_future;
use spikard_http::{BackgroundHandle, BackgroundJobError, BackgroundJobMetadata};
use std::sync::RwLock;

static BACKGROUND_HANDLE: Lazy<RwLock<Option<BackgroundHandle>>> = Lazy::new(|| RwLock::new(None));

pub fn install_handle(handle: BackgroundHandle) {
    match BACKGROUND_HANDLE.write() {
        Ok(mut guard) => *guard = Some(handle),
        Err(poisoned) => *poisoned.into_inner() = Some(handle),
    }
}

pub fn clear_handle() {
    match BACKGROUND_HANDLE.write() {
        Ok(mut guard) => *guard = None,
        Err(poisoned) => *poisoned.into_inner() = None,
    }
}

#[pyfunction]
pub fn background_run(awaitable: Bound<'_, PyAny>) -> PyResult<()> {
    let handle = BACKGROUND_HANDLE
        .read()
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("background handle poisoned"))?
        .clone()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("background runtime not initialized"))?;

    if !awaitable.hasattr("__await__")? {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "background.run expects an awaitable",
        ));
    }

    let future = into_future(awaitable)?;
    handle
        .spawn_with_metadata(
            async move {
                match future.await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(BackgroundJobError::from(format_pyerr(err))),
                }
            },
            BackgroundJobMetadata::default(),
        )
        .map_err(|err| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string()))?;

    Ok(())
}

fn format_pyerr(err: PyErr) -> String {
    Python::attach(|py| {
        err.into_value(py)
            .bind(py)
            .repr()
            .ok()
            .and_then(|repr| repr.extract::<String>().ok())
            .unwrap_or_else(|| "Background task failed".to_string())
    })
}
