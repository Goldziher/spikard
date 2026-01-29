//! `WASIp3` HTTP component for Spikard framework.
//!
//! This crate implements a `wasi:http/service` component that serves HTTP requests
//! using Spikard's core validation and routing engine. It is designed to run on
//! any WASI-compatible runtime (e.g., `wasmtime serve`).
//!
//! # Architecture
//!
//! The component receives HTTP requests via the `wasi:http/handler` interface,
//! routes them through compiled route tables, applies validation using
//! `spikard_core`, and returns structured responses.

mod adapter;
mod auth;
mod config;
mod rate_limit;
mod router;

use adapter::{from_wasi_request, to_wasi_response};
use router::Router;
use wasip3::http::types::{Request, Response, ErrorCode};

wasip3::http::service::export!(SpikardComponent);

struct SpikardComponent;

impl wasip3::exports::http::handler::Guest for SpikardComponent {
    async fn handle(request: Request) -> Result<Response, ErrorCode> {
        // Convert WASI request to our internal representation
        let internal_request = from_wasi_request(request).await?;

        // Route the request
        let router = Router::global();
        let handler_result = router.dispatch(&internal_request);

        // Convert response back to WASI types
        to_wasi_response(handler_result)
    }
}
