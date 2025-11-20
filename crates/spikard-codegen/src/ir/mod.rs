//! Intermediate representation for code generation

use crate::error::Result;
use crate::parser::Config;
use serde::{Deserialize, Serialize};

/// Intermediate representation of the configuration
///
/// This provides a normalized view of the configuration that's easier
/// to work with during code generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediateRepresentation {
    pub service_name: String,
    pub routes: Vec<RouteIR>,
    pub schemas: Vec<SchemaIR>,
    // TODO: Add gRPC, queues, CloudEvents
}

/// Intermediate representation of a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteIR {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub parameters: ParametersIR,
    pub request_body: Option<SchemaIR>,
    pub response: ResponseIR,
}

/// Parameters for a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersIR {
    pub path: Vec<ParameterIR>,
    pub query: Vec<ParameterIR>,
    pub headers: Vec<ParameterIR>,
    pub cookies: Vec<ParameterIR>,
}

/// Single parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterIR {
    pub name: String,
    pub schema: SchemaIR,
    pub required: bool,
}

/// Response specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseIR {
    pub status_code: u16,
    pub schema: Option<SchemaIR>,
}

/// Schema representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaIR {
    pub name: Option<String>,
    pub schema_type: String,
    pub properties: Vec<PropertyIR>,
    pub required: Vec<String>,
}

/// Schema property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyIR {
    pub name: String,
    pub property_type: String,
    pub format: Option<String>,
    pub constraints: Vec<ConstraintIR>,
}

/// Validation constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintIR {
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Minimum(f64),
    Maximum(f64),
    MinItems(usize),
    MaxItems(usize),
}

impl IntermediateRepresentation {
    /// Convert configuration to intermediate representation
    pub fn from_config(config: Config) -> Result<Self> {
        let mut routes = Vec::new();

        if let Some(http) = config.http {
            for route in http.routes {
                routes.push(RouteIR::from_config_route(route)?);
            }
        }

        Ok(Self {
            service_name: config.name,
            routes,
            schemas: Vec::new(), // TODO: Extract from schemas section
        })
    }
}

impl RouteIR {
    fn from_config_route(route: crate::parser::config::HttpRoute) -> Result<Self> {
        // TODO: Fully implement conversion
        Ok(Self {
            path: route.path,
            method: route.method,
            handler: route.handler,
            parameters: ParametersIR {
                path: Vec::new(),
                query: Vec::new(),
                headers: Vec::new(),
                cookies: Vec::new(),
            },
            request_body: None,
            response: ResponseIR {
                status_code: 200,
                schema: None,
            },
        })
    }
}
