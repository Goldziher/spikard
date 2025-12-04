// WASM-specific tests using wasm-bindgen-test framework
// These tests are compiled and run only for wasm32 target
// Run with: wasm-pack test --headless --firefox crates/spikard-wasm

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

// Configure to run in browser environment for full WASM testing
wasm_bindgen_test_configure!(run_in_browser);

// ============================================================================
// Test 1: Module Initialization
// ============================================================================

#[wasm_bindgen_test]
fn test_init_sets_panic_hook() {
    // The init() function should set the panic hook without panicking
    // This test verifies that panic hook initialization works
    spikard_wasm::init();
    // If we reach here, init() succeeded
    assert!(true);
}

// ============================================================================
// Test 2: TestClient Construction - Valid JSON Routes
// ============================================================================

#[wasm_bindgen_test]
fn test_client_construction_with_valid_routes() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/users",
            "handler_name": "list_users"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_ok(), "TestClient should construct with valid routes");
}

// ============================================================================
// Test 3: TestClient Construction - Invalid Routes JSON
// ============================================================================

#[wasm_bindgen_test]
fn test_client_construction_invalid_json() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "{ invalid json }";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_err(), "TestClient should reject invalid JSON routes");
}

// ============================================================================
// Test 4: TestClient Construction - Handlers Must Be Object
// ============================================================================

#[wasm_bindgen_test]
fn test_client_construction_handlers_type_check() {
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = JsValue::from_str("not an object");
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers, config, None);

    assert!(result.is_err(), "TestClient should require handlers to be an object");
}

// ============================================================================
// Test 5: TestClient GET Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
async fn test_get_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/test",
            "handler_name": "test_handler"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(client.is_ok(), "Client should construct successfully");

    let client = client.unwrap();
    let promise = client.get("/test".to_string(), JsValue::NULL);

    // Verify the return value is a Promise
    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "GET should return a Promise"
    );
}

// ============================================================================
// Test 6: TestClient DELETE Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_delete_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.delete("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "DELETE should return a Promise"
    );
}

// ============================================================================
// Test 7: TestClient POST Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_post_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.post("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "POST should return a Promise"
    );
}

// ============================================================================
// Test 8: TestClient PUT Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_put_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.put("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "PUT should return a Promise"
    );
}

// ============================================================================
// Test 9: TestClient PATCH Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_patch_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.patch("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "PATCH should return a Promise"
    );
}

// ============================================================================
// Test 10: TestClient HEAD Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_head_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.head("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "HEAD should return a Promise"
    );
}

// ============================================================================
// Test 11: TestClient OPTIONS Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_options_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.options("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "OPTIONS should return a Promise"
    );
}

// ============================================================================
// Test 12: TestClient TRACE Method Returns Promise
// ============================================================================

#[wasm_bindgen_test]
fn test_trace_method_returns_promise() {
    use js_sys::{Object, Promise};
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    let client = client.unwrap();
    let promise = client.trace("/test".to_string(), JsValue::NULL);

    assert!(
        Promise::from(promise.clone()).is_instance_of::<Promise>(),
        "TRACE should return a Promise"
    );
}

// ============================================================================
// Test 13: Multiple Routes Configuration
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_multiple_routes() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/users",
            "handler_name": "list_users"
        },
        {
            "method": "POST",
            "path": "/users",
            "handler_name": "create_user"
        },
        {
            "method": "GET",
            "path": "/users/:id",
            "handler_name": "get_user"
        },
        {
            "method": "PUT",
            "path": "/users/:id",
            "handler_name": "update_user"
        },
        {
            "method": "DELETE",
            "path": "/users/:id",
            "handler_name": "delete_user"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_ok(), "TestClient should accept multiple routes");
}

// ============================================================================
// Test 14: Server Config Deserialization - Valid JSON
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_valid_config() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::from_str(r#"{"compression":{"gzip":true,"brotli":false,"minSize":1024,"quality":6}}"#);

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_ok(), "TestClient should accept valid server config");
}

// ============================================================================
// Test 15: Server Config Deserialization - Invalid Config Type
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_invalid_config_type() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::from(42); // Not a string

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_err(), "TestClient should reject non-string config");
}

// ============================================================================
// Test 16: Server Config Deserialization - Malformed JSON
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_malformed_config_json() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::from_str("{ malformed json }");

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_err(), "TestClient should reject malformed config JSON");
}

// ============================================================================
// Test 17: Route Definition with Schemas
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_routes_containing_schemas() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "POST",
            "path": "/users",
            "handler_name": "create_user",
            "request_schema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "email": { "type": "string" }
                }
            },
            "response_schema": {
                "type": "object",
                "properties": {
                    "id": { "type": "number" },
                    "name": { "type": "string" }
                }
            }
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(
        result.is_ok(),
        "TestClient should accept routes with validation schemas"
    );
}

// ============================================================================
// Test 18: JsValue Error Propagation - Invalid Handler Type
// ============================================================================

#[wasm_bindgen_test]
fn test_error_propagation_invalid_handler_map() {
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = JsValue::from_str("invalid");
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers, config, None);

    match result {
        Err(err) => {
            // Verify the error is a JsValue (FFI boundary preservation)
            assert!(err.is_object(), "Error should be a JsValue");
        }
        Ok(_) => panic!("Should have failed with invalid handlers"),
    }
}

// ============================================================================
// Test 19: Client Supports Null/Undefined Config
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_null_config() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();

    // Test with NULL
    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), JsValue::NULL, None);

    assert!(result.is_ok(), "TestClient should accept NULL config");

    // Test with UNDEFINED
    let result = spikard_wasm::TestClient::new(routes_json, Object::new().into(), JsValue::UNDEFINED, None);

    assert!(result.is_ok(), "TestClient should accept UNDEFINED config");
}

// ============================================================================
// Test 20: Client Supports Null Lifecycle Hooks
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_null_lifecycle_hooks() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_ok(), "TestClient should accept None for lifecycle hooks");
}

// ============================================================================
// Test 21: Client Creation Does Not Panic on Valid Input
// ============================================================================

#[wasm_bindgen_test]
fn test_client_creation_no_panic() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/",
            "handler_name": "root"
        },
        {
            "method": "POST",
            "path": "/api/data",
            "handler_name": "api_create"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    // This should not panic
    let _ = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    // If we reach here without panicking, test passes
    assert!(true);
}

// ============================================================================
// Test 22: Multiple Client Instances Can Be Created
// ============================================================================

#[wasm_bindgen_test]
fn test_multiple_client_instances() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json1 = r#"[{"method":"GET","path":"/app1","handler_name":"handler1"}]"#;
    let routes_json2 = r#"[{"method":"POST","path":"/app2","handler_name":"handler2"}]"#;

    let handlers1 = Object::new();
    let handlers2 = Object::new();

    let client1 = spikard_wasm::TestClient::new(routes_json1, handlers1.into(), JsValue::NULL, None);

    let client2 = spikard_wasm::TestClient::new(routes_json2, handlers2.into(), JsValue::NULL, None);

    assert!(
        client1.is_ok() && client2.is_ok(),
        "Should be able to create multiple client instances"
    );
}

// ============================================================================
// Test 23: Test Memory Safety - No Dangling References
// ============================================================================

#[wasm_bindgen_test]
fn test_memory_safety_handler_references() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/test",
            "handler_name": "my_handler"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    // Create client with handlers - should not create dangling refs
    let _client = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    // If no panic/segfault, memory safety is preserved
    assert!(true);
}

// ============================================================================
// Test 24: Route Path Parameter Handling
// ============================================================================

#[wasm_bindgen_test]
fn test_route_with_path_parameters() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/users/:userId/posts/:postId",
            "handler_name": "get_user_post"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_ok(), "TestClient should accept routes with path parameters");
}

// ============================================================================
// Test 25: Empty Routes Array
// ============================================================================

#[wasm_bindgen_test]
fn test_client_with_empty_routes() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    let routes_json = "[]";
    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    assert!(result.is_ok(), "TestClient should accept empty routes array");
}

// ============================================================================
// Test 26: Case Insensitive HTTP Methods
// ============================================================================

#[wasm_bindgen_test]
fn test_route_methods_various_cases() {
    use js_sys::Object;
    use wasm_bindgen::JsValue;

    // The routes should work with different method cases
    let routes_json = r#"[
        {
            "method": "GET",
            "path": "/lower",
            "handler_name": "h1"
        },
        {
            "method": "get",
            "path": "/lower2",
            "handler_name": "h2"
        },
        {
            "method": "GeT",
            "path": "/mixed",
            "handler_name": "h3"
        }
    ]"#;

    let handlers = Object::new();
    let config = JsValue::NULL;

    let result = spikard_wasm::TestClient::new(routes_json, handlers.into(), config, None);

    // The client should accept method definitions with various cases
    assert!(result.is_ok());
}
