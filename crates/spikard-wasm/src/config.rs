//! Server configuration for the WASI HTTP component.
//!
//! Configuration is compiled into the component at build time,
//! or loaded from environment at startup.

use serde::{Deserialize, Serialize};

/// Configuration for the Spikard WASI component.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Routes to register.
    #[serde(default)]
    pub routes: Vec<RouteConfig>,

    /// Rate limiting configuration.
    #[serde(default)]
    pub rate_limit: Option<RateLimitConfig>,
}

/// Configuration for a single route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    pub method: String,
    pub path: String,
    pub handler_id: String,
}

/// Rate limiting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_tokens: f64,
    pub refill_rate: f64,
}

