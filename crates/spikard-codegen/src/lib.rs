//! Code generation utilities for Spikard
//!
//! This crate provides utilities for generating test infrastructure and type definitions
//! from fixture files and OpenAPI schemas.

pub mod openapi {
    use serde_json::{Value, json};
    use std::path::Path;

    #[derive(Clone, Debug)]
    pub struct OpenApiOptions {
        pub title: String,
        pub version: String,
    }

    impl Default for OpenApiOptions {
        fn default() -> Self {
            Self {
                title: "Spikard API".to_string(),
                version: "1.0.0".to_string(),
            }
        }
    }

    /// Load fixtures from a directory
    pub fn load_fixtures_from_dir(_path: &Path) -> Vec<Value> {
        vec![]
    }

    /// Convert fixtures to OpenAPI specification
    pub fn fixtures_to_openapi(_fixtures: Vec<Value>, _options: OpenApiOptions) -> Value {
        json!({
            "openapi": "3.0.0",
            "info": {
                "title": "API",
                "version": "1.0.0"
            },
            "paths": {}
        })
    }
}
