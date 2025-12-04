#![deny(clippy::unwrap_used)]
#![cfg_attr(all(windows, target_env = "msvc", feature = "extension-module"), feature(abi_vectorcall))]
#[cfg(feature = "extension-module")]
mod php;

#[cfg(feature = "extension-module")]
pub use php::*;

#[cfg(not(feature = "extension-module"))]
pub fn spikard_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(not(feature = "extension-module"))]
pub fn spikard_echo_response(body: &str) -> String {
    body.to_string()
}
