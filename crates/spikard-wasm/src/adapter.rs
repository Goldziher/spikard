//! Adapter between wasi:http types and internal representations.
//!
//! Uses the `wasip3` http-compat feature to bridge between WASI resource-based
//! HTTP types and standard Rust `http` crate types, then converts to our
//! internal request/response structs for routing.

use crate::router::HandlerResult;
use wasip3::http::types::{ErrorCode, Request as WasiRequest, Response as WasiResponse};
/// Internal request representation, decoupled from WASI types.
#[derive(Debug)]
pub struct InternalRequest {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Convert a WASI incoming request to our internal representation.
pub async fn from_wasi_request(request: WasiRequest) -> Result<InternalRequest, ErrorCode> {
    let method = match request.get_method() {
        wasip3::http::types::Method::Get => "GET",
        wasip3::http::types::Method::Head => "HEAD",
        wasip3::http::types::Method::Post => "POST",
        wasip3::http::types::Method::Put => "PUT",
        wasip3::http::types::Method::Delete => "DELETE",
        wasip3::http::types::Method::Connect => "CONNECT",
        wasip3::http::types::Method::Options => "OPTIONS",
        wasip3::http::types::Method::Trace => "TRACE",
        wasip3::http::types::Method::Patch => "PATCH",
        wasip3::http::types::Method::Other(ref s) => s.as_str(),
    }
    .to_string();

    let (path, query) = request.get_path_with_query().map_or_else(
        || ("/".to_string(), None),
        |pq| match pq.find('?') {
            Some(idx) => (pq[..idx].to_string(), Some(pq[idx + 1..].to_string())),
            None => (pq, None),
        },
    );

    let headers_resource = request.get_headers();
    let raw_headers = headers_resource.copy_all();
    let headers: Vec<(String, String)> = raw_headers
        .into_iter()
        .filter_map(|(name, value)| String::from_utf8(value).ok().map(|v| (name, v)))
        .collect();

    // Use http-compat IncomingMessage trait to consume the body
    let http_request = wasip3::http_compat::http_from_wasi_request(request)
        .map_err(|e| ErrorCode::InternalError(Some(format!("request conversion: {e:?}"))))?;

    let body = http_body_util::BodyExt::collect(http_request.into_body())
        .await
        .map(|collected| collected.to_bytes().to_vec())
        .unwrap_or_default();

    Ok(InternalRequest {
        method,
        path,
        query,
        headers,
        body,
    })
}

/// Convert a handler result to a WASI response.
pub fn to_wasi_response(result: HandlerResult) -> Result<WasiResponse, ErrorCode> {
    let mut builder = http::Response::builder().status(result.status);

    for (key, value) in &result.headers {
        builder = builder.header(key.as_str(), value.as_str());
    }

    let http_response = builder
        .body(http_body_util::Full::new(bytes::Bytes::from(result.body)))
        .map_err(|e| ErrorCode::InternalError(Some(format!("response build: {e}"))))?;

    wasip3::http_compat::http_into_wasi_response(http_response)
        .map_err(|e| ErrorCode::InternalError(Some(format!("response conversion: {e:?}"))))
}
