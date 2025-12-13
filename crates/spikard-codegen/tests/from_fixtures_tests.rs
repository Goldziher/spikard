use spikard_codegen::openapi::from_fixtures::{OpenApiOptions, fixtures_to_openapi, load_fixtures_from_dir};
use std::fs;

#[test]
fn loads_fixture_files_and_builds_openapi_spec() {
    let dir = tempfile::tempdir().expect("tempdir");
    let fixtures_dir = dir.path();

    let fixture = serde_json::json!({
        "name": "health_ok",
        "description": "Health check returns ok",
        "handler": {
            "route": "/health",
            "method": "GET",
            "response_schema": { "type": "object", "properties": { "status": { "type": "string" } }, "required": ["status"] }
        },
        "request": {
            "method": "GET",
            "path": "/health"
        },
        "expected_response": {
            "status_code": 200,
            "body": { "status": "ok" }
        }
    });

    fs::write(
        fixtures_dir.join("01-health.json"),
        serde_json::to_vec_pretty(&fixture).unwrap(),
    )
    .unwrap();

    let loaded = load_fixtures_from_dir(fixtures_dir).expect("load");
    assert_eq!(loaded.len(), 1);

    let spec = fixtures_to_openapi(
        loaded,
        OpenApiOptions {
            title: "Fixture API".to_string(),
            version: "0.1.0".to_string(),
            description: None,
        },
    )
    .expect("spec");

    assert_eq!(spec.openapi, "3.1.0");
    assert!(spec.paths.contains_key("/health"));
    let path = spec.paths.get("/health").unwrap();
    assert!(path.get.is_some());
}

#[test]
fn fixture_loader_skips_schema_and_prefixed_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let fixtures_dir = dir.path();

    fs::write(fixtures_dir.join("schema.json"), "{}").unwrap();
    fs::write(fixtures_dir.join("00-ignored.json"), "{}").unwrap();
    fs::write(
        fixtures_dir.join("01-valid.json"),
        serde_json::to_vec(&serde_json::json!({
            "name": "ok",
            "description": "ok",
            "request": { "method": "GET", "path": "/" },
            "expected_response": { "status_code": 200 }
        }))
        .unwrap(),
    )
    .unwrap();

    let loaded = load_fixtures_from_dir(fixtures_dir).expect("load");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].name, "ok");
}

#[test]
fn fixtures_to_openapi_extracts_parameters_and_request_body() {
    let fixtures = vec![spikard_codegen::openapi::from_fixtures::Fixture {
        name: "create_user".to_string(),
        description: "Create user".to_string(),
        category: None,
        handler: Some(spikard_codegen::openapi::from_fixtures::FixtureHandler {
            route: "/users/{id}".to_string(),
            method: "POST".to_string(),
            parameters: Some(serde_json::json!({
                "path": {
                    "id": { "type": "string", "description": "user id" }
                },
                "query": {
                    "verbose": { "type": "boolean", "required": false }
                },
                "headers": {
                    "x-token": { "type": "string", "required": true }
                },
                "cookies": {
                    "session": { "type": "string" }
                }
            })),
            body_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "minLength": 1 },
                    "age": { "type": "integer", "minimum": 0 }
                },
                "required": ["name"]
            })),
            response_schema: None,
            cors: None,
            middleware: None,
            dependencies: None,
            handler_dependencies: None,
            route_overrides: None,
            injection_strategy: None,
        }),
        streaming: None,
        background: None,
        request: spikard_codegen::openapi::from_fixtures::FixtureRequest {
            method: "POST".to_string(),
            path: "/users/123".to_string(),
            query_params: None,
            headers: None,
            cookies: None,
            body: None,
            data: None,
            form_data: None,
            files: None,
            content_type: None,
        },
        expected_response: spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse {
            status_code: 201,
            body: Some(serde_json::json!({"ok":true})),
            body_partial: None,
            headers: None,
            validation_errors: None,
        },
        tags: Some(vec!["users".to_string()]),
    }];

    let spec = fixtures_to_openapi(fixtures, OpenApiOptions::default()).expect("spec");
    let path_item = spec.paths.get("/users/123").expect("path");
    let op = path_item.post.as_ref().expect("post");

    let params = op.parameters.as_ref().expect("parameters");
    assert!(params.iter().any(|p| p.location == "path" && p.name == "id"));
    assert!(params.iter().any(|p| p.location == "query" && p.name == "verbose"));
    assert!(params.iter().any(|p| p.location == "header" && p.name == "x-token"));
    assert!(params.iter().any(|p| p.location == "cookie" && p.name == "session"));

    let body = op.request_body.as_ref().expect("request_body");
    assert!(body.content.contains_key("application/json"));
}
