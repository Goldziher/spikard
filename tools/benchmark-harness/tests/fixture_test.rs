//! Unit tests for fixture loading

use benchmark_harness::fixture::{Fixture, FixtureManager};
use serde_json::json;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_fixture_from_json() {
    let fixture_json = json!({
        "name": "simple_get",
        "description": "Simple GET request",
        "category": "query_params",
        "handler": {
            "route": "/test",
            "method": "GET",
            "parameters": {}
        },
        "request": {
            "method": "GET",
            "path": "/test",
            "query_params": {},
            "headers": {},
            "cookies": {}
        },
        "expected_response": {
            "status_code": 200,
            "body": {"message": "ok"}
        }
    });

    let fixture: Fixture = serde_json::from_value(fixture_json).unwrap();
    assert_eq!(fixture.name, "simple_get");
    assert_eq!(fixture.description, "Simple GET request");
    assert_eq!(fixture.category, Some("query_params".to_string()));
    assert_eq!(fixture.handler.route, "/test");
    assert_eq!(fixture.handler.method, "GET");
    assert_eq!(fixture.request.method, "GET");
    assert_eq!(fixture.request.path, "/test");
    assert_eq!(fixture.expected_response.status_code, 200);
}

#[test]
fn test_fixture_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let fixture_path = temp_dir.path().join("test_fixture.json");

    let fixture_json = json!({
        "name": "test_fixture",
        "description": "Test fixture",
        "handler": {
            "route": "/test",
            "method": "POST"
        },
        "request": {
            "method": "POST",
            "path": "/test",
            "body": {"key": "value"}
        },
        "expected_response": {
            "status_code": 201
        }
    });

    std::fs::write(&fixture_path, serde_json::to_string_pretty(&fixture_json).unwrap()).unwrap();

    let fixture = Fixture::from_file(&fixture_path).unwrap();
    assert_eq!(fixture.name, "test_fixture");
    assert_eq!(fixture.handler.method, "POST");
    assert!(fixture.request.body.is_some());
}

#[test]
fn test_fixture_from_file_invalid_json() {
    let temp_dir = TempDir::new().unwrap();
    let fixture_path = temp_dir.path().join("invalid.json");

    std::fs::write(&fixture_path, "{ invalid json }").unwrap();

    let result = Fixture::from_file(&fixture_path);
    assert!(result.is_err());
}

#[test]
fn test_fixture_from_file_missing() {
    let result = Fixture::from_file("/nonexistent/fixture.json");
    assert!(result.is_err());
}

#[test]
fn test_fixture_from_dir() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple fixture files
    for i in 1..=3 {
        let fixture_json = json!({
            "name": format!("fixture_{}", i),
            "description": format!("Fixture {}", i),
            "handler": {
                "route": format!("/test{}", i),
                "method": "GET"
            },
            "request": {
                "method": "GET",
                "path": format!("/test{}", i)
            },
            "expected_response": {
                "status_code": 200
            }
        });

        let path = temp_dir.path().join(format!("fixture_{}.json", i));
        std::fs::write(&path, serde_json::to_string_pretty(&fixture_json).unwrap()).unwrap();
    }

    // Create a schema.json file (should be skipped)
    let schema_json = json!({"type": "object"});
    let schema_path = temp_dir.path().join("schema.json");
    std::fs::write(&schema_path, serde_json::to_string_pretty(&schema_json).unwrap()).unwrap();

    let fixtures = Fixture::from_dir(temp_dir.path()).unwrap();
    assert_eq!(fixtures.len(), 3); // schema.json should be skipped
}

#[test]
fn test_fixture_from_glob() {
    let temp_dir = TempDir::new().unwrap();

    // Create fixtures in subdirectories
    let query_dir = temp_dir.path().join("query_params");
    std::fs::create_dir(&query_dir).unwrap();

    let fixture_json = json!({
        "name": "query_fixture",
        "description": "Query param fixture",
        "handler": {
            "route": "/search",
            "method": "GET"
        },
        "request": {
            "method": "GET",
            "path": "/search",
            "query_params": {"q": "test"}
        },
        "expected_response": {
            "status_code": 200
        }
    });

    let path = query_dir.join("query_fixture.json");
    std::fs::write(&path, serde_json::to_string_pretty(&fixture_json).unwrap()).unwrap();

    let fixtures = Fixture::from_glob("query_params/*.json", temp_dir.path()).unwrap();
    assert_eq!(fixtures.len(), 1);
    assert_eq!(fixtures[0].name, "query_fixture");
}

#[test]
fn test_fixture_category() {
    let mut fixture: Fixture = serde_json::from_value(json!({
        "name": "test",
        "description": "Test",
        "handler": {"route": "/", "method": "GET"},
        "request": {"method": "GET", "path": "/"},
        "expected_response": {"status_code": 200}
    }))
    .unwrap();

    // No category set
    assert_eq!(fixture.category(), "unknown");

    // Set category
    fixture.category = Some("test_category".to_string());
    assert_eq!(fixture.category(), "test_category");
}

#[test]
fn test_fixture_with_query_params() {
    let mut query_params = HashMap::new();
    query_params.insert("name".to_string(), json!("test"));
    query_params.insert("age".to_string(), json!(25));

    let fixture_json = json!({
        "name": "query_test",
        "description": "Query param test",
        "handler": {
            "route": "/search",
            "method": "GET",
            "parameters": {
                "query": query_params
            }
        },
        "request": {
            "method": "GET",
            "path": "/search",
            "query_params": query_params
        },
        "expected_response": {
            "status_code": 200
        }
    });

    let fixture: Fixture = serde_json::from_value(fixture_json).unwrap();
    assert_eq!(fixture.request.query_params.len(), 2);
    assert_eq!(fixture.request.query_params.get("name"), Some(&json!("test")));
}

#[test]
fn test_fixture_with_headers_and_cookies() {
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token".to_string());

    let mut cookies = HashMap::new();
    cookies.insert("session".to_string(), "abc123".to_string());

    let fixture_json = json!({
        "name": "auth_test",
        "description": "Auth test",
        "handler": {
            "route": "/api",
            "method": "GET"
        },
        "request": {
            "method": "GET",
            "path": "/api",
            "headers": headers,
            "cookies": cookies
        },
        "expected_response": {
            "status_code": 200
        }
    });

    let fixture: Fixture = serde_json::from_value(fixture_json).unwrap();
    assert_eq!(fixture.request.headers.len(), 1);
    assert_eq!(fixture.request.cookies.len(), 1);
    assert_eq!(
        fixture.request.headers.get("Authorization"),
        Some(&"Bearer token".to_string())
    );
}

#[test]
fn test_fixture_manager_new() {
    let manager = FixtureManager::new();
    assert!(manager.is_empty());
    assert_eq!(manager.len(), 0);
}

#[test]
fn test_fixture_manager_by_category() {
    let mut manager = FixtureManager::new();

    let fixture1: Fixture = serde_json::from_value(json!({
        "name": "fixture1",
        "description": "Fixture 1",
        "category": "query_params",
        "handler": {"route": "/", "method": "GET"},
        "request": {"method": "GET", "path": "/"},
        "expected_response": {"status_code": 200}
    }))
    .unwrap();

    let fixture2: Fixture = serde_json::from_value(json!({
        "name": "fixture2",
        "description": "Fixture 2",
        "category": "json_bodies",
        "handler": {"route": "/", "method": "POST"},
        "request": {"method": "POST", "path": "/"},
        "expected_response": {"status_code": 200}
    }))
    .unwrap();

    manager.fixtures.push(fixture1);
    manager.fixtures.push(fixture2);

    let query_fixtures = manager.by_category("query_params");
    assert_eq!(query_fixtures.len(), 1);
    assert_eq!(query_fixtures[0].name, "fixture1");

    let json_fixtures = manager.by_category("json_bodies");
    assert_eq!(json_fixtures.len(), 1);
    assert_eq!(json_fixtures[0].name, "fixture2");
}

#[test]
fn test_fixture_manager_all() {
    let mut manager = FixtureManager::new();

    let fixture: Fixture = serde_json::from_value(json!({
        "name": "test",
        "description": "Test",
        "handler": {"route": "/", "method": "GET"},
        "request": {"method": "GET", "path": "/"},
        "expected_response": {"status_code": 200}
    }))
    .unwrap();

    manager.fixtures.push(fixture);

    let all = manager.all();
    assert_eq!(all.len(), 1);
}

#[test]
fn test_fixture_manager_load_from_testing_data_nonexistent() {
    let mut manager = FixtureManager::new();

    // Try to load from a directory that doesn't exist
    let temp_dir = TempDir::new().unwrap();
    let result = manager.load_from_testing_data(temp_dir.path().join("nonexistent"));

    // Should not error, just not load any fixtures
    assert!(result.is_ok());
    assert!(manager.is_empty());
}
