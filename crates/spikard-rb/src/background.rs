use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, Ruby, Value};
use once_cell::sync::Lazy;
use spikard_http::{BackgroundHandle, BackgroundJobError, BackgroundJobMetadata};
use std::sync::{Arc, RwLock};

static BACKGROUND_HANDLE: Lazy<RwLock<Option<BackgroundHandle>>> = Lazy::new(|| RwLock::new(None));

pub fn install_handle(handle: BackgroundHandle) {
    *BACKGROUND_HANDLE.write().expect("background handle lock poisoned") = Some(handle);
}

pub fn clear_handle() {
    *BACKGROUND_HANDLE.write().expect("background handle lock poisoned") = None;
}

pub fn background_run(ruby: &Ruby, block: Value) -> Result<(), Error> {
    let call_sym = ruby.intern("call");
    if !block.respond_to(call_sym, false)? {
        return Err(Error::new(
            ruby.exception_type_error(),
            "background.run expects a callable block",
        ));
    }

    let handle = BACKGROUND_HANDLE
        .read()
        .map_err(|_| Error::new(ruby.exception_runtime_error(), "background handle lock poisoned"))?
        .clone()
        .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "background runtime not initialized"))?;

    let proc_value = Arc::new(Opaque::from(block));

    handle
        .spawn_with_metadata(
            async move {
                let proc_clone = proc_value.clone();
                tokio::task::spawn_blocking(move || -> Result<(), BackgroundJobError> {
                    let ruby = Ruby::get().map_err(|e| BackgroundJobError::from(e.to_string()))?;
                    let callable = proc_clone.get_inner_with(&ruby);
                    callable
                        .funcall::<_, _, Value>("call", ())
                        .map(|_| ())
                        .map_err(|err| BackgroundJobError::from(format_ruby_error(err)))
                })
                .await
                .map_err(|e| BackgroundJobError::from(e.to_string()))?
            },
            BackgroundJobMetadata::default(),
        )
        .map_err(|err| Error::new(ruby.exception_runtime_error(), err.to_string()))
}

fn format_ruby_error(err: Error) -> String {
    err.to_string()
}
