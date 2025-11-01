//! Spikard HTTP Server
//!
//! Core HTTP server implementation with route management and validation

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod cors;
pub mod debug;
pub mod handler;
pub mod middleware;
pub mod parameters;
pub mod problem;
pub mod query_parser;
pub mod response;
pub mod router;
pub mod server;
pub mod validation;

pub use handler::PythonHandler;
pub use parameters::ParameterValidator;
pub use problem::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};
pub use response::Response;
pub use router::{Route, RouteHandler, Router};
pub use server::Server;
pub use validation::SchemaValidator;

/// HTTP method
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
        }
    }
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "PATCH" => Ok(Method::Patch),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            "OPTIONS" => Ok(Method::Options),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }
}

/// CORS configuration for a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}

/// Route metadata extracted from Python
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteMetadata {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    pub request_schema: Option<Value>,
    pub response_schema: Option<Value>,
    pub parameter_schema: Option<Value>,
    pub is_async: bool,
    pub cors: Option<CorsConfig>,
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            workers: 1,
        }
    }
}
