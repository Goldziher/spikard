use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::into_future;
use spikard_http::{BackgroundHandle, BackgroundJobError, BackgroundJobMetadata, BackgroundSpawnError};
use std::sync::RwLock;

static BACKGROUND_HANDLE: Lazy<RwLock<Option<BackgroundHandle>>> = Lazy::new(|| RwLock::new(None));

pub fn install_handle(handle: BackgroundHandle) {
    *BACKGROUND_HANDLE.write().unwrap() = Some(handle);
}

pub fn clear_handle() {
    *BACKGROUND_HANDLE.write().unwrap() = None;
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
        .map_err(map_spawn_error)?;

    Ok(())
}

fn map_spawn_error(err: BackgroundSpawnError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())
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
