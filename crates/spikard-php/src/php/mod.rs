//! ext-php-rs implementation.

use ext_php_rs::prelude::*;

mod request;
mod response;

pub use request::PhpRequest;
pub use response::PhpResponse;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function("spikard_version", spikard_version)
        .function("spikard_echo_response", spikard_echo_response)
        .class::<PhpRequest>()
        .class::<PhpResponse>()
}

/// Return the crate version.
#[php_function]
pub fn spikard_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Placeholder entrypoint: echo a response for sanity checks.
#[php_function]
pub fn spikard_echo_response(body: &str) -> PhpResponse {
    PhpResponse::json(body.into(), Some(200))
}
