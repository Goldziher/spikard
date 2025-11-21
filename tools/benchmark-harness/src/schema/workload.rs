//! Workload definitions and suite system

use serde::{Deserialize, Serialize};

/// HTTP endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub method: String, // "GET", "POST", etc.
    pub path: String,   // "/json/small", "/items/{id}"
}

/// Workload definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadDef {
    pub name: String,
    pub description: String,
    pub category: String, // "json-bodies", "path-params", etc.
    pub endpoint: Endpoint,
    pub payload_size_bytes: Option<u64>,
    pub body_file: Option<String>,    // Path to request body
    pub content_type: Option<String>, // "application/json", etc.
}

/// Workload suite definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSuite {
    pub name: String,
    pub description: String,
    pub workloads: Vec<WorkloadDef>,
}

impl WorkloadSuite {
    /// Get all built-in workload definitions
    pub fn all() -> Self {
        Self {
            name: "all".to_string(),
            description: "All workloads".to_string(),
            workloads: vec![
                // JSON Bodies
                Self::json_small(),
                Self::json_medium(),
                Self::json_large(),
                Self::json_very_large(),
                // Path Parameters
                Self::path_simple(),
                Self::path_multiple(),
                Self::path_deep(),
                Self::path_int(),
                Self::path_uuid(),
                Self::path_date(),
                // Query Parameters
                Self::query_few(),
                Self::query_medium(),
                Self::query_many(),
                // URL-Encoded Forms
                Self::urlencoded_simple(),
                Self::urlencoded_complex(),
                // Multipart File Uploads
                Self::multipart_small(),
                Self::multipart_medium(),
                Self::multipart_large(),
            ],
        }
    }

    /// JSON bodies suite
    pub fn json_bodies() -> Self {
        Self {
            name: "json-bodies".to_string(),
            description: "JSON serialization workloads".to_string(),
            workloads: vec![
                Self::json_small(),
                Self::json_medium(),
                Self::json_large(),
                Self::json_very_large(),
            ],
        }
    }

    /// Path parameters suite
    pub fn path_params() -> Self {
        Self {
            name: "path-params".to_string(),
            description: "Path parameter extraction".to_string(),
            workloads: vec![
                Self::path_simple(),
                Self::path_multiple(),
                Self::path_deep(),
                Self::path_int(),
                Self::path_uuid(),
                Self::path_date(),
            ],
        }
    }

    /// Query parameters suite
    pub fn query_params() -> Self {
        Self {
            name: "query-params".to_string(),
            description: "Query string parsing".to_string(),
            workloads: vec![Self::query_few(), Self::query_medium(), Self::query_many()],
        }
    }

    /// Forms suite
    pub fn forms() -> Self {
        Self {
            name: "forms".to_string(),
            description: "Form data handling".to_string(),
            workloads: vec![Self::urlencoded_simple(), Self::urlencoded_complex()],
        }
    }

    /// Multipart suite
    pub fn multipart() -> Self {
        Self {
            name: "multipart".to_string(),
            description: "Multipart file uploads".to_string(),
            workloads: vec![
                Self::multipart_small(),
                Self::multipart_medium(),
                Self::multipart_large(),
            ],
        }
    }

    // Individual workload definitions

    fn json_small() -> WorkloadDef {
        WorkloadDef {
            name: "json-small".to_string(),
            description: "Small JSON payload (~86 bytes)".to_string(),
            category: "json-bodies".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/json/small".to_string(),
            },
            payload_size_bytes: Some(86),
            body_file: Some("json-small.json".to_string()),
            content_type: Some("application/json".to_string()),
        }
    }

    fn json_medium() -> WorkloadDef {
        WorkloadDef {
            name: "json-medium".to_string(),
            description: "Medium JSON payload (~1.5 KB)".to_string(),
            category: "json-bodies".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/json/medium".to_string(),
            },
            payload_size_bytes: Some(1536),
            body_file: Some("json-medium.json".to_string()),
            content_type: Some("application/json".to_string()),
        }
    }

    fn json_large() -> WorkloadDef {
        WorkloadDef {
            name: "json-large".to_string(),
            description: "Large JSON payload (~15 KB)".to_string(),
            category: "json-bodies".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/json/large".to_string(),
            },
            payload_size_bytes: Some(15360),
            body_file: Some("json-large.json".to_string()),
            content_type: Some("application/json".to_string()),
        }
    }

    fn json_very_large() -> WorkloadDef {
        WorkloadDef {
            name: "json-very-large".to_string(),
            description: "Very large JSON payload (~150 KB)".to_string(),
            category: "json-bodies".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/json/very-large".to_string(),
            },
            payload_size_bytes: Some(153600),
            body_file: Some("json-very-large.json".to_string()),
            content_type: Some("application/json".to_string()),
        }
    }

    fn path_simple() -> WorkloadDef {
        WorkloadDef {
            name: "path-simple".to_string(),
            description: "Single path parameter".to_string(),
            category: "path-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/path/simple/test123".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn path_multiple() -> WorkloadDef {
        WorkloadDef {
            name: "path-multiple".to_string(),
            description: "Multiple path parameters".to_string(),
            category: "path-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/path/multiple/user456/post789".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn path_deep() -> WorkloadDef {
        WorkloadDef {
            name: "path-deep".to_string(),
            description: "Deep path hierarchy (5 levels)".to_string(),
            category: "path-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/path/deep/acme/engineering/backend/api/item123".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn path_int() -> WorkloadDef {
        WorkloadDef {
            name: "path-int".to_string(),
            description: "Integer path parameter".to_string(),
            category: "path-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/path/int/42".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn path_uuid() -> WorkloadDef {
        WorkloadDef {
            name: "path-uuid".to_string(),
            description: "UUID path parameter".to_string(),
            category: "path-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/path/uuid/550e8400-e29b-41d4-a716-446655440000".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn path_date() -> WorkloadDef {
        WorkloadDef {
            name: "path-date".to_string(),
            description: "Date path parameter".to_string(),
            category: "path-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/path/date/2024-01-15".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn query_few() -> WorkloadDef {
        WorkloadDef {
            name: "query-few".to_string(),
            description: "Few query parameters (3)".to_string(),
            category: "query-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/query/few?q=search&page=1&limit=20".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn query_medium() -> WorkloadDef {
        WorkloadDef {
            name: "query-medium".to_string(),
            description: "Medium query parameters (8)".to_string(),
            category: "query-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/query/medium?category=electronics&tags=phone,smart&min_price=100&max_price=1000&sort=price&order=asc&page=1&limit=20".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn query_many() -> WorkloadDef {
        WorkloadDef {
            name: "query-many".to_string(),
            description: "Many query parameters (15+)".to_string(),
            category: "query-params".to_string(),
            endpoint: Endpoint {
                method: "GET".to_string(),
                path: "/query/many?q=search&page=1&limit=20&sort=date&order=desc&filter=active&category=tech&subcategory=mobile&brand=test&min_price=0&max_price=999&rating=4&verified=true&in_stock=true&shipping=fast&color=blue".to_string(),
            },
            payload_size_bytes: None,
            body_file: None,
            content_type: None,
        }
    }

    fn urlencoded_simple() -> WorkloadDef {
        WorkloadDef {
            name: "urlencoded-simple".to_string(),
            description: "Simple URL-encoded form (4 fields)".to_string(),
            category: "forms".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/urlencoded/simple".to_string(),
            },
            payload_size_bytes: Some(60),
            body_file: Some("urlencoded-simple.txt".to_string()),
            content_type: Some("application/x-www-form-urlencoded".to_string()),
        }
    }

    fn urlencoded_complex() -> WorkloadDef {
        WorkloadDef {
            name: "urlencoded-complex".to_string(),
            description: "Complex URL-encoded form (18 fields)".to_string(),
            category: "forms".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/urlencoded/complex".to_string(),
            },
            payload_size_bytes: Some(300),
            body_file: Some("urlencoded-complex.txt".to_string()),
            content_type: Some("application/x-www-form-urlencoded".to_string()),
        }
    }

    fn multipart_small() -> WorkloadDef {
        WorkloadDef {
            name: "multipart-small".to_string(),
            description: "Small multipart file upload (~1 KB)".to_string(),
            category: "multipart".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/multipart/small".to_string(),
            },
            payload_size_bytes: Some(1024),
            body_file: Some("multipart-small.bin".to_string()),
            content_type: Some("multipart/form-data".to_string()),
        }
    }

    fn multipart_medium() -> WorkloadDef {
        WorkloadDef {
            name: "multipart-medium".to_string(),
            description: "Medium multipart file upload (~10 KB)".to_string(),
            category: "multipart".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/multipart/medium".to_string(),
            },
            payload_size_bytes: Some(10240),
            body_file: Some("multipart-medium.bin".to_string()),
            content_type: Some("multipart/form-data".to_string()),
        }
    }

    fn multipart_large() -> WorkloadDef {
        WorkloadDef {
            name: "multipart-large".to_string(),
            description: "Large multipart file upload (~100 KB)".to_string(),
            category: "multipart".to_string(),
            endpoint: Endpoint {
                method: "POST".to_string(),
                path: "/multipart/large".to_string(),
            },
            payload_size_bytes: Some(102400),
            body_file: Some("multipart-large.bin".to_string()),
            content_type: Some("multipart/form-data".to_string()),
        }
    }

    /// Load suite by name
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "all" => Some(Self::all()),
            "json-bodies" => Some(Self::json_bodies()),
            "path-params" => Some(Self::path_params()),
            "query-params" => Some(Self::query_params()),
            "forms" => Some(Self::forms()),
            "multipart" => Some(Self::multipart()),
            _ => None,
        }
    }
}
