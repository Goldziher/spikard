use once_cell::sync::Lazy;
use std::sync::RwLock;

use napi::Error;
use napi::bindgen_prelude::{Function, Promise, Result};
use napi_derive::napi;
use spikard_http::{BackgroundHandle, BackgroundJobError, BackgroundJobMetadata, BackgroundSpawnError};

static BACKGROUND_HANDLE: Lazy<RwLock<Option<BackgroundHandle>>> = Lazy::new(|| RwLock::new(None));

pub fn install_handle(handle: BackgroundHandle) {
    match BACKGROUND_HANDLE.write() {
        Ok(mut guard) => *guard = Some(handle),
        Err(_) => eprintln!("warning: background handle lock poisoned, continuing"),
    }
}

pub fn clear_handle() {
    match BACKGROUND_HANDLE.write() {
        Ok(mut guard) => *guard = None,
        Err(_) => eprintln!("warning: background handle lock poisoned, continuing"),
    }
}

#[napi]
/// Run an async task on Spikard's background runtime.
pub fn background_run(task: Function<(), Promise<()>>) -> Result<()> {
    let handle = BACKGROUND_HANDLE
        .read()
        .map_err(|_| Error::from_reason("background handle lock poisoned"))?
        .clone()
        .ok_or_else(|| Error::from_reason("background runtime not initialized"))?;

    let tsfn = task
        .build_threadsafe_function()
        .build_callback(|ctx| {
            let _: () = ctx.value;
            Ok(())
        })
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

fn map_spawn_error(err: BackgroundSpawnError) -> Error {
    Error::from_reason(err.to_string())
}
