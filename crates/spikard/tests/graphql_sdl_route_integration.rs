//! Integration tests for `App::register_graphql_sdl_route`.
//!
//! One test per fixture case in `fixtures/graphql_operations.json`, exercising the
//! dynamic-SDL GraphQL executor end to end via an in-process HTTP client. Each test asserts
//! the exact response body from `fixtures/graphql_operations.json`'s `expected_response.body`,
//! except where that fixture uses the `<<present>>` wildcard (engine-version-dependent
//! messages), in which case the test asserts only that the field is present and non-empty.

use serde_json::json;
use spikard::testing::test_client_from_app;
use spikard::{App, DynamicSchemaConfig, FieldErrorSpec, Method};

async fn post_graphql(
    sdl: &str,
    response_data: serde_json::Value,
    config: &DynamicSchemaConfig,
    query: &str,
    variables: Option<serde_json::Value>,
    operation_name: Option<&str>,
) -> serde_json::Value {
    let mut app = App::new();
    app.register_graphql_sdl_route("/graphql", Method::Post, sdl, response_data, config)
        .expect("register dynamic graphql route");

    let client = test_client_from_app(app).expect("test client from app");
    let snapshot = client
        .graphql(query, variables, operation_name)
        .await
        .expect("POST /graphql");
    assert_eq!(snapshot.status, 200);
    snapshot.json().expect("parse json body")
}

fn enabled_config() -> DynamicSchemaConfig {
    DynamicSchemaConfig {
        introspection_enabled: true,
        ..Default::default()
    }
}

/// `graphql_simple_query`
#[tokio::test]
async fn test_graphql_simple_query() {
    let body = post_graphql(
        "type Query { hello: String! }",
        json!({"hello": "world"}),
        &enabled_config(),
        "{ hello }",
        None,
        None,
    )
    .await;

    assert_eq!(body, json!({"data": {"hello": "world"}}));
}

/// `graphql_query_with_variables`
#[tokio::test]
async fn test_graphql_query_with_variables() {
    let body = post_graphql(
        "type Query { user(id: ID!): User } type User { id: ID! name: String! }",
        json!({"user": {"id": "user-42", "name": "Alice"}}),
        &enabled_config(),
        "query GetUser($id: ID!) { user(id: $id) { id name } }",
        Some(json!({"id": "user-42"})),
        None,
    )
    .await;

    assert_eq!(body, json!({"data": {"user": {"id": "user-42", "name": "Alice"}}}));
}

/// `graphql_query_nested_fields`
#[tokio::test]
async fn test_graphql_query_nested_fields() {
    let sdl = "type Query { order(id: ID!): Order } type Order { id: ID! customer: Customer items: [OrderItem!]! } type Customer { id: ID! name: String! address: Address } type Address { street: String! city: String! } type OrderItem { sku: String! quantity: Int! }";
    let response_data = json!({
        "order": {
            "id": "ORD-1",
            "customer": {
                "id": "CUST-1",
                "name": "Bob",
                "address": {"street": "1 Main St", "city": "Springfield"}
            },
            "items": [{"sku": "WIDGET-A", "quantity": 2}]
        }
    });

    let body = post_graphql(
        sdl,
        response_data,
        &enabled_config(),
        "{ order(id: \"ORD-1\") { id customer { name address { city } } items { sku quantity } } }",
        None,
        None,
    )
    .await;

    assert_eq!(
        body,
        json!({
            "data": {
                "order": {
                    "id": "ORD-1",
                    "customer": {
                        "name": "Bob",
                        "address": {"city": "Springfield"}
                    },
                    "items": [{"sku": "WIDGET-A", "quantity": 2}]
                }
            }
        })
    );
}

/// `graphql_mutation_create_object`
#[tokio::test]
async fn test_graphql_mutation_create_object() {
    let sdl = "type Mutation { createUser(input: CreateUserInput!): User! } input CreateUserInput { name: String! email: String! } type User { id: ID! name: String! email: String! } type Query { _placeholder: Boolean }";
    let response_data = json!({
        "createUser": {"id": "new-user-id", "name": "Carol", "email": "carol@example.com"}
    });

    let body = post_graphql(
        sdl,
        response_data,
        &enabled_config(),
        "mutation CreateUser($input: CreateUserInput!) { createUser(input: $input) { id name email } }",
        Some(json!({"input": {"name": "Carol", "email": "carol@example.com"}})),
        None,
    )
    .await;

    assert_eq!(
        body,
        json!({
            "data": {
                "createUser": {"id": "new-user-id", "name": "Carol", "email": "carol@example.com"}
            }
        })
    );
}

/// `graphql_subscription_declaration`
///
/// Genuine `async-graphql` behavior: `Schema::execute` (the synchronous request/response
/// transport used by a POST handler) unconditionally rejects `subscription` operations with
/// "Subscriptions are not supported on this transport." — subscriptions only execute via
/// `Schema::execute_stream`, which requires a streaming transport (WebSocket via
/// `graphql-transport-ws`). The SDL and subscription root ARE registered and validated for real
/// (this test also proves that), but a `subscription { ... }` document posted over plain HTTP
/// cannot return the operation's data — reworking this fixture to require a data payload would
/// require faking the response instead of executing it. See the dynamic-SDL executor's
/// `register_type`/`build_subscription_field` in `spikard-graphql::dynamic` for the (unused by
/// this transport) subscription resolver wiring kept for a future WebSocket entry point.
#[tokio::test]
async fn test_graphql_subscription_declaration() {
    let sdl = "type Subscription { messageAdded(roomId: ID!): Message! } type Message { id: ID! text: String! authorId: ID! } type Query { _placeholder: Boolean }";
    let response_data = json!({
        "messageAdded": {"id": "msg-1", "text": "Hello room", "authorId": "user-1"}
    });

    let body = post_graphql(
        sdl,
        response_data,
        &enabled_config(),
        "subscription OnMessageAdded($roomId: ID!) { messageAdded(roomId: $roomId) { id text authorId } }",
        Some(json!({"roomId": "room-1"})),
        None,
    )
    .await;

    assert_eq!(
        body,
        json!({
            "data": null,
            "errors": [{"message": "Subscriptions are not supported on this transport."}]
        })
    );
}

/// `graphql_introspection_query`
#[tokio::test]
async fn test_graphql_introspection_query() {
    let body = post_graphql(
        "type Query { hello: String! }",
        json!({"hello": "world"}),
        &enabled_config(),
        "{ __schema { queryType { name } } }",
        None,
        None,
    )
    .await;

    assert_eq!(body, json!({"data": {"__schema": {"queryType": {"name": "Query"}}}}));
}

/// `graphql_invalid_syntax_rejected`
///
/// The fixture uses `<<present>>` for the error message: `async-graphql`'s parser emits a
/// version-specific, pretty-printed pest error rather than a fixed "Syntax Error" string.
#[tokio::test]
async fn test_graphql_invalid_syntax_rejected() {
    let body = post_graphql(
        "type Query { hello: String! }",
        json!({"hello": "world"}),
        &enabled_config(),
        "{ hello { this is not valid syntax {{{",
        None,
        None,
    )
    .await;

    assert_eq!(body["data"], serde_json::Value::Null);
    let errors = body["errors"].as_array().expect("errors array");
    assert_eq!(errors.len(), 1);
    let message = errors[0]["message"].as_str().expect("message is a string");
    assert!(!message.is_empty(), "<<present>>: message must be non-empty");
}

/// `graphql_ops_complexity_limit_exceeded`
#[tokio::test]
async fn test_graphql_ops_complexity_limit_exceeded() {
    let mut config = enabled_config();
    config.max_complexity = Some(10);

    let body = post_graphql(
        "type Query { users: [User!]! } type User { id: ID! friends: [User!]! }",
        json!({"users": []}),
        &config,
        "{ users { id friends { id friends { id friends { id friends { id friends { id } } } } } } }",
        None,
        None,
    )
    .await;

    assert_eq!(
        body,
        json!({
            "data": null,
            "errors": [{"message": "Query is too complex."}]
        })
    );
}

/// `graphql_ops_depth_limit_exceeded`
#[tokio::test]
async fn test_graphql_ops_depth_limit_exceeded() {
    let mut config = enabled_config();
    config.max_depth = Some(3);

    let sdl = "type Query { a: Level1 } type Level1 { b: Level2 } type Level2 { c: Level3 } type Level3 { d: Level4 } type Level4 { e: String }";
    let response_data = json!({"a": {"b": {"c": {"d": {"e": "deep"}}}}});

    let body = post_graphql(sdl, response_data, &config, "{ a { b { c { d { e } } } } }", None, None).await;

    assert_eq!(
        body,
        json!({
            "data": null,
            "errors": [{"message": "Query is nested too deep."}]
        })
    );
}

/// `graphql_field_level_error_partial_data`
#[tokio::test]
async fn test_graphql_field_level_error_partial_data() {
    let mut config = enabled_config();
    config.field_errors.push(FieldErrorSpec {
        path: "user".to_string(),
        message: "User not found".to_string(),
    });

    let sdl = "type Query { user(id: ID!): User status: String! } type User { id: ID! name: String! avatar: String }";
    let response_data = json!({"user": null, "status": "ok"});

    let body = post_graphql(
        sdl,
        response_data,
        &config,
        "{ user(id: \"missing-user\") { id name avatar } status }",
        None,
        None,
    )
    .await;

    assert_eq!(
        body,
        json!({
            "data": {"user": null, "status": "ok"},
            "errors": [{
                "message": "User not found",
                "locations": [{"line": 1, "column": 3}],
                "path": ["user"]
            }]
        })
    );
}

/// `graphql_multi_operation_with_operation_name`
#[tokio::test]
async fn test_graphql_multi_operation_with_operation_name() {
    let body = post_graphql(
        "type Query { ping: String! version: String! }",
        json!({"ping": "pong", "version": "1.0.0"}),
        &enabled_config(),
        "query Ping { ping } query Version { version }",
        None,
        Some("Version"),
    )
    .await;

    assert_eq!(body, json!({"data": {"version": "1.0.0"}}));
}

/// `graphql_undefined_field_error`
#[tokio::test]
async fn test_graphql_undefined_field_error() {
    let body = post_graphql(
        "type Query { hello: String! }",
        json!({"hello": "world"}),
        &enabled_config(),
        "{ hello nonExistentField }",
        None,
        None,
    )
    .await;

    assert_eq!(
        body,
        json!({
            "data": null,
            "errors": [{
                "message": "Unknown field \"nonExistentField\" on type \"Query\".",
                "locations": [{"line": 1, "column": 9}]
            }]
        })
    );
}

/// Sanity check that `register_graphql_sdl_route` rejects malformed SDL rather than silently
/// registering a broken route.
#[tokio::test]
async fn test_invalid_sdl_is_rejected_at_registration() {
    let mut app = App::new();
    let result = app.register_graphql_sdl_route(
        "/graphql",
        Method::Post,
        "not valid sdl {{{",
        json!({}),
        &enabled_config(),
    );

    assert!(result.is_err());
}
