use once_cell::sync::Lazy;
use std::sync::RwLock;

use napi::Error;
use napi::bindgen_prelude::{Function, Promise, Result};
use napi_derive::napi;
use spikard_http::{BackgroundHandle, BackgroundJobError, BackgroundJobMetadata, BackgroundSpawnError};

static BACKGROUND_HANDLE: Lazy<RwLock<Option<BackgroundHandle>>> = Lazy::new(|| RwLock::new(None));

pub fn install_handle(handle: BackgroundHandle) {
    *BACKGROUND_HANDLE.write().expect("background handle lock poisoned") = Some(handle);
}

pub fn clear_handle() {
    *BACKGROUND_HANDLE.write().expect("background handle lock poisoned") = None;
}

#[napi]
#[allow(dead_code)]
pub fn background_run(task: Function<(), Promise<()>>) -> Result<()> {
    let handle = BACKGROUND_HANDLE
        .read()
        .map_err(|_| Error::from_reason("background handle lock poisoned"))?
        .clone()
        .ok_or_else(|| Error::from_reason("background runtime not initialized"))?;

    let tsfn = task
        .build_threadsafe_function()
        .build_callback(|ctx| Ok(vec![ctx.value]))
        .map_err(|e| Error::from_reason(format!("Failed to build background callback: {}", e)))?;

    handle
        .spawn_with_metadata(
            async move {
                tsfn.call_async(())
                    .await
                    .map_err(|e| BackgroundJobError::from(e.to_string()))?
                    .await
                    .map_err(|e| BackgroundJobError::from(e.to_string()))?;
                Ok(())
            },
            BackgroundJobMetadata::default(),
        )
        .map_err(map_spawn_error)
}

#[allow(dead_code)]
fn map_spawn_error(err: BackgroundSpawnError) -> Error {
    Error::from_reason(err.to_string())
}
