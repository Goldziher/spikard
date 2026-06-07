//! Spikard's HTTP annotation grammar, parsed out of scythe's `CustomAnnotation`
//! slice. Scythe captures every unknown `-- @<name> <value>` line verbatim;
//! spikard owns the vocabulary that turns those triples into route metadata.

use std::collections::BTreeMap;

use scythe_core::analyzer::AnalyzedQuery;
use scythe_core::parser::CustomAnnotation;
use scythe_core::parser::QueryCommand;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors raised while parsing HTTP annotations. Each variant carries the
/// 1-based source line from the originating `CustomAnnotation` so messages can
/// point users at the offending SQL.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnnotationParseError {
    #[error("line {line}: @http expects '<METHOD> <PATH>' (got '{value}')")]
    MalformedHttp { line: usize, value: String },

    #[error("line {line}: unknown HTTP method '{method}'")]
    UnknownMethod { line: usize, method: String },

    #[error("line {line}: duplicate @http directive (only one route per query)")]
    DuplicateHttp { line: usize },

    #[error("line {line}: @http_param expects '<name> <path|query|body|header>' (got '{value}')")]
    MalformedHttpParam { line: usize, value: String },

    #[error("line {line}: unknown @http_param binding '{binding}' (expected path/query/body/header)")]
    UnknownBinding { line: usize, binding: String },

    #[error("line {line}: @http_status expects comma-separated codes (got '{value}')")]
    MalformedHttpStatus { line: usize, value: String },

    #[error(
        "line {line}: @http_auth expects 'none', 'bearer[:<format>]', or 'api_key:<location>:<name>' (got '{value}')"
    )]
    MalformedHttpAuth { line: usize, value: String },

    #[error("line {line}: @http_auth api_key location must be header/query/cookie (got '{location}')")]
    UnknownApiKeyLocation { line: usize, location: String },

    #[error(
        "command :{command} cannot be mapped to HTTP (only :one, :opt, :many, :exec, :exec_rows, :grouped are supported)"
    )]
    IncompatibleCommand { command: String },

    #[error("command :{command} requires method {expected_methods:?} (got {actual_method})")]
    MethodCommandMismatch {
        command: String,
        expected_methods: Vec<&'static str>,
        actual_method: String,
    },
}

/// HTTP method extracted from `@http <METHOD> <PATH>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Some(Self::Get),
            "POST" => Some(Self::Post),
            "PUT" => Some(Self::Put),
            "PATCH" => Some(Self::Patch),
            "DELETE" => Some(Self::Delete),
            "HEAD" => Some(Self::Head),
            "OPTIONS" => Some(Self::Options),
            _ => None,
        }
    }
}

/// Where an HTTP request parameter is sourced from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HttpParamBinding {
    Path,
    Query,
    Body,
    Header,
}

/// Authentication requirement attached to a route, mapping directly to
/// spikard's existing `SecuritySchemeInfo` enum (bearer-style HTTP auth or
/// API-key auth).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AuthRequirement {
    None,
    Bearer {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
    },
    ApiKey {
        location: ApiKeyLocation,
        name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyLocation {
    Header,
    Query,
    Cookie,
}

/// Parsed HTTP metadata for a single SQL query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpAnnotations {
    pub method: HttpMethod,
    /// Path normalized to spikard's canonical `{name}` form. Both `:id` and
    /// `{id}` are accepted in source and emitted as `{id}`.
    pub path: String,
    /// Explicit param-location overrides, keyed by parameter name. Names
    /// absent from this map fall back to inference rules (see
    /// [`bin_param_locations`](crate::sql::route)).
    pub param_bindings: BTreeMap<String, HttpParamBinding>,
    /// Name of the bundled body object when multiple body params exist.
    pub request_body_name: Option<String>,
    /// Status codes the route documents (defaults derived from the SQL
    /// `QueryCommand` when empty).
    pub status_codes: Vec<u16>,
    pub auth: Option<AuthRequirement>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
}

/// Parse spikard's HTTP vocabulary out of the custom-annotation slice that
/// scythe captured. Returns `Ok(None)` when no `@http` directive is present —
/// queries without HTTP semantics co-exist in the same source tree.
pub fn parse_http_annotations(custom: &[CustomAnnotation]) -> Result<Option<HttpAnnotations>, AnnotationParseError> {
    let mut http: Option<(usize, HttpMethod, String)> = None;
    let mut param_bindings: BTreeMap<String, HttpParamBinding> = BTreeMap::new();
    let mut request_body_name: Option<String> = None;
    let mut status_codes: Vec<u16> = Vec::new();
    let mut auth: Option<AuthRequirement> = None;
    let mut tags: Vec<String> = Vec::new();
    let mut summary: Option<String> = None;
    let mut description: Option<String> = None;

    for ann in custom {
        match ann.name.as_str() {
            "http" => {
                if http.is_some() {
                    return Err(AnnotationParseError::DuplicateHttp { line: ann.line });
                }
                let (method_raw, path_raw) =
                    ann.value
                        .split_once(char::is_whitespace)
                        .ok_or_else(|| AnnotationParseError::MalformedHttp {
                            line: ann.line,
                            value: ann.value.clone(),
                        })?;
                let method = HttpMethod::from_str(method_raw).ok_or_else(|| AnnotationParseError::UnknownMethod {
                    line: ann.line,
                    method: method_raw.to_string(),
                })?;
                let path = normalize_path(path_raw.trim());
                if path.is_empty() {
                    return Err(AnnotationParseError::MalformedHttp {
                        line: ann.line,
                        value: ann.value.clone(),
                    });
                }
                http = Some((ann.line, method, path));
            }
            "http_param" => {
                let (name, binding_raw) = ann.value.split_once(char::is_whitespace).ok_or_else(|| {
                    AnnotationParseError::MalformedHttpParam {
                        line: ann.line,
                        value: ann.value.clone(),
                    }
                })?;
                let binding =
                    parse_binding(binding_raw.trim()).ok_or_else(|| AnnotationParseError::UnknownBinding {
                        line: ann.line,
                        binding: binding_raw.trim().to_string(),
                    })?;
                param_bindings.insert(name.trim().to_string(), binding);
            }
            "http_request_body" => {
                let trimmed = ann.value.trim();
                if !trimmed.is_empty() {
                    request_body_name = Some(trimmed.to_string());
                }
            }
            "http_status" => {
                for code_raw in ann.value.split(',') {
                    let trimmed = code_raw.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let code = trimmed
                        .parse::<u16>()
                        .map_err(|_| AnnotationParseError::MalformedHttpStatus {
                            line: ann.line,
                            value: ann.value.clone(),
                        })?;
                    status_codes.push(code);
                }
            }
            "http_auth" => {
                auth = Some(parse_auth(&ann.value, ann.line)?);
            }
            "http_tags" => {
                for tag in ann.value.split(',') {
                    let trimmed = tag.trim();
                    if !trimmed.is_empty() {
                        tags.push(trimmed.to_string());
                    }
                }
            }
            "http_summary" => {
                summary = Some(ann.value.trim().to_string()).filter(|s| !s.is_empty());
            }
            "http_description" => {
                description = Some(ann.value.trim().to_string()).filter(|s| !s.is_empty());
            }
            // Annotations spikard doesn't recognise are ignored here — they
            // belong to some other consumer layered on top of scythe.
            _ => {}
        }
    }

    let Some((_, method, path)) = http else {
        return Ok(None);
    };

    Ok(Some(HttpAnnotations {
        method,
        path,
        param_bindings,
        request_body_name,
        status_codes,
        auth,
        tags,
        summary,
        description,
    }))
}

/// Validate that the HTTP method declared on a query is compatible with the
/// scythe `QueryCommand`, and return the default status code for the (command,
/// method) pair when [`HttpAnnotations::status_codes`] is empty.
pub fn default_status_for(command: &QueryCommand, method: HttpMethod) -> Result<u16, AnnotationParseError> {
    let (allowed, default): (&[HttpMethod], u16) = match command {
        QueryCommand::One | QueryCommand::Opt | QueryCommand::Many | QueryCommand::Grouped => (&[HttpMethod::Get], 200),
        QueryCommand::Exec => (
            &[HttpMethod::Post, HttpMethod::Put, HttpMethod::Patch, HttpMethod::Delete],
            204,
        ),
        QueryCommand::ExecRows => (
            &[HttpMethod::Post, HttpMethod::Put, HttpMethod::Patch, HttpMethod::Delete],
            200,
        ),
        QueryCommand::ExecResult | QueryCommand::Batch => {
            return Err(AnnotationParseError::IncompatibleCommand {
                command: command.to_string(),
            });
        }
    };

    if !allowed.contains(&method) {
        return Err(AnnotationParseError::MethodCommandMismatch {
            command: command.to_string(),
            expected_methods: allowed.iter().map(|m| m.as_str()).collect(),
            actual_method: method.as_str().to_string(),
        });
    }
    Ok(default)
}

/// Convenience: parse the HTTP annotations on an `AnalyzedQuery` AND validate
/// the command/method combination in one call. Returns `Ok(None)` when the
/// query has no `@http` directive.
pub fn parse_for_query(query: &AnalyzedQuery) -> Result<Option<(HttpAnnotations, u16)>, AnnotationParseError> {
    let Some(http) = parse_http_annotations(&query.custom)? else {
        return Ok(None);
    };
    let default_status = default_status_for(&query.command, http.method)?;
    Ok(Some((http, default_status)))
}

fn parse_binding(s: &str) -> Option<HttpParamBinding> {
    match s.to_ascii_lowercase().as_str() {
        "path" => Some(HttpParamBinding::Path),
        "query" => Some(HttpParamBinding::Query),
        "body" => Some(HttpParamBinding::Body),
        "header" => Some(HttpParamBinding::Header),
        _ => None,
    }
}

fn parse_auth(value: &str, line: usize) -> Result<AuthRequirement, AnnotationParseError> {
    let trimmed = value.trim();
    if trimmed.eq_ignore_ascii_case("none") {
        return Ok(AuthRequirement::None);
    }
    if let Some(rest) = trimmed
        .strip_prefix("bearer")
        .or_else(|| trimmed.strip_prefix("Bearer"))
    {
        let rest = rest.trim();
        if rest.is_empty() {
            return Ok(AuthRequirement::Bearer { format: None });
        }
        if let Some(format) = rest.strip_prefix(':') {
            let format = format.trim();
            if format.is_empty() {
                return Ok(AuthRequirement::Bearer { format: None });
            }
            return Ok(AuthRequirement::Bearer {
                format: Some(format.to_string()),
            });
        }
        return Err(AnnotationParseError::MalformedHttpAuth {
            line,
            value: value.to_string(),
        });
    }
    if let Some(rest) = trimmed
        .strip_prefix("api_key")
        .or_else(|| trimmed.strip_prefix("apikey"))
    {
        let rest = rest
            .strip_prefix(':')
            .ok_or_else(|| AnnotationParseError::MalformedHttpAuth {
                line,
                value: value.to_string(),
            })?;
        let (location_raw, name) = rest
            .split_once(':')
            .ok_or_else(|| AnnotationParseError::MalformedHttpAuth {
                line,
                value: value.to_string(),
            })?;
        let location = match location_raw.trim().to_ascii_lowercase().as_str() {
            "header" => ApiKeyLocation::Header,
            "query" => ApiKeyLocation::Query,
            "cookie" => ApiKeyLocation::Cookie,
            other => {
                return Err(AnnotationParseError::UnknownApiKeyLocation {
                    line,
                    location: other.to_string(),
                });
            }
        };
        return Ok(AuthRequirement::ApiKey {
            location,
            name: name.trim().to_string(),
        });
    }
    Err(AnnotationParseError::MalformedHttpAuth {
        line,
        value: value.to_string(),
    })
}

/// Normalize an `@http` path so colon-prefixed placeholders (`:id`) become the
/// brace-wrapped form (`{id}`) that spikard uses canonically. The brace form
/// passes through unchanged.
fn normalize_path(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let bytes = raw.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b':' && i + 1 < bytes.len() && is_ident_start(bytes[i + 1]) {
            out.push('{');
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                out.push(bytes[i] as char);
                i += 1;
            }
            out.push('}');
        } else {
            out.push(b as char);
            i += 1;
        }
    }
    out
}

const fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

const fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use scythe_core::parser::CustomAnnotation;

    fn ann(name: &str, value: &str, line: usize) -> CustomAnnotation {
        CustomAnnotation {
            name: name.to_string(),
            value: value.to_string(),
            line,
        }
    }

    #[test]
    fn returns_none_when_no_http_directive() {
        let custom = vec![ann("http_auth", "bearer", 1)];
        assert_eq!(parse_http_annotations(&custom).unwrap(), None);
    }

    #[test]
    fn parses_basic_get_route() {
        let custom = vec![ann("http", "GET /users/{id}", 3)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.method, HttpMethod::Get);
        assert_eq!(h.path, "/users/{id}");
    }

    #[test]
    fn normalizes_colon_placeholders_to_braces() {
        let custom = vec![ann("http", "GET /users/:id/orders/:order_id", 1)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.path, "/users/{id}/orders/{order_id}");
    }

    #[test]
    fn leaves_brace_placeholders_unchanged() {
        let custom = vec![ann("http", "GET /users/{id}/orders/{order_id}", 1)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.path, "/users/{id}/orders/{order_id}");
    }

    #[test]
    fn rejects_duplicate_http_directives() {
        let custom = vec![ann("http", "GET /a", 1), ann("http", "GET /b", 2)];
        assert!(matches!(
            parse_http_annotations(&custom).unwrap_err(),
            AnnotationParseError::DuplicateHttp { line: 2 }
        ));
    }

    #[test]
    fn rejects_unknown_method() {
        let custom = vec![ann("http", "FETCH /users", 4)];
        assert!(matches!(
            parse_http_annotations(&custom).unwrap_err(),
            AnnotationParseError::UnknownMethod { line: 4, .. }
        ));
    }

    #[test]
    fn parses_param_bindings() {
        let custom = vec![
            ann("http", "POST /users", 1),
            ann("http_param", "id path", 2),
            ann("http_param", "email body", 3),
            ann("http_param", "limit query", 4),
        ];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.param_bindings.get("id"), Some(&HttpParamBinding::Path));
        assert_eq!(h.param_bindings.get("email"), Some(&HttpParamBinding::Body));
        assert_eq!(h.param_bindings.get("limit"), Some(&HttpParamBinding::Query));
    }

    #[test]
    fn rejects_unknown_binding() {
        let custom = vec![ann("http", "POST /x", 1), ann("http_param", "id foo", 5)];
        assert!(matches!(
            parse_http_annotations(&custom).unwrap_err(),
            AnnotationParseError::UnknownBinding { line: 5, .. }
        ));
    }

    #[test]
    fn parses_status_codes() {
        let custom = vec![ann("http", "GET /a", 1), ann("http_status", "200, 404", 2)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.status_codes, vec![200, 404]);
    }

    #[test]
    fn parses_bearer_auth() {
        let custom = vec![ann("http", "GET /a", 1), ann("http_auth", "bearer", 2)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.auth, Some(AuthRequirement::Bearer { format: None }));
    }

    #[test]
    fn parses_bearer_with_format() {
        let custom = vec![ann("http", "GET /a", 1), ann("http_auth", "bearer:jwt", 2)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(
            h.auth,
            Some(AuthRequirement::Bearer {
                format: Some("jwt".to_string()),
            })
        );
    }

    #[test]
    fn parses_api_key_auth() {
        let custom = vec![
            ann("http", "GET /a", 1),
            ann("http_auth", "api_key:header:X-API-Key", 2),
        ];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(
            h.auth,
            Some(AuthRequirement::ApiKey {
                location: ApiKeyLocation::Header,
                name: "X-API-Key".to_string(),
            })
        );
    }

    #[test]
    fn parses_none_auth() {
        let custom = vec![ann("http", "GET /a", 1), ann("http_auth", "none", 2)];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.auth, Some(AuthRequirement::None));
    }

    #[test]
    fn rejects_unknown_auth_scheme() {
        let custom = vec![ann("http", "GET /a", 1), ann("http_auth", "oauth2:scopes", 7)];
        assert!(matches!(
            parse_http_annotations(&custom).unwrap_err(),
            AnnotationParseError::MalformedHttpAuth { line: 7, .. }
        ));
    }

    #[test]
    fn parses_tags_and_summary() {
        let custom = vec![
            ann("http", "GET /a", 1),
            ann("http_tags", "users, admin ", 2),
            ann("http_summary", "List users", 3),
            ann("http_description", "Returns every user", 4),
        ];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.tags, vec!["users", "admin"]);
        assert_eq!(h.summary.as_deref(), Some("List users"));
        assert_eq!(h.description.as_deref(), Some("Returns every user"));
    }

    #[test]
    fn ignores_unrelated_annotations() {
        let custom = vec![
            ann("http", "GET /a", 1),
            ann("gql_field", "user.email", 2),
            ann("queue", "background", 3),
        ];
        let h = parse_http_annotations(&custom).unwrap().unwrap();
        assert_eq!(h.method, HttpMethod::Get);
    }

    #[test]
    fn default_status_one_get() {
        assert_eq!(default_status_for(&QueryCommand::One, HttpMethod::Get).unwrap(), 200);
    }

    #[test]
    fn default_status_exec_post() {
        assert_eq!(default_status_for(&QueryCommand::Exec, HttpMethod::Post).unwrap(), 204);
    }

    #[test]
    fn default_status_exec_rows_put() {
        assert_eq!(
            default_status_for(&QueryCommand::ExecRows, HttpMethod::Put).unwrap(),
            200
        );
    }

    #[test]
    fn rejects_batch_command() {
        assert!(matches!(
            default_status_for(&QueryCommand::Batch, HttpMethod::Get),
            Err(AnnotationParseError::IncompatibleCommand { .. })
        ));
    }

    #[test]
    fn rejects_exec_result_command() {
        assert!(matches!(
            default_status_for(&QueryCommand::ExecResult, HttpMethod::Post),
            Err(AnnotationParseError::IncompatibleCommand { .. })
        ));
    }

    #[test]
    fn rejects_one_with_post() {
        assert!(matches!(
            default_status_for(&QueryCommand::One, HttpMethod::Post),
            Err(AnnotationParseError::MethodCommandMismatch { .. })
        ));
    }

    #[test]
    fn rejects_exec_with_get() {
        assert!(matches!(
            default_status_for(&QueryCommand::Exec, HttpMethod::Get),
            Err(AnnotationParseError::MethodCommandMismatch { .. })
        ));
    }
}
