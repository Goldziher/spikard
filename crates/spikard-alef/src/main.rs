//! spikard-alef — alef CLI with the spikard HTTP extension registered.
//!
//! Wraps `alef::run_with_extensions` and adds `HttpExtension` so that spikard's
//! HTTP-domain IR (lifecycle hooks, WebSocket/SSE routes, error types) is included
//! in every generation run without requiring any changes to alef core.

fn main() -> std::process::ExitCode {
    alef::run_with_extensions(vec![Box::new(spikard_alef_ext::HttpExtension::new())])
}
