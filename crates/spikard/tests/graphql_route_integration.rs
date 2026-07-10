//! Integration tests for `App::register_graphql_route`.
//!
//! Exercises the built-in async-graphql test schemas end to end via an
//! in-process HTTP client, asserting the exact response bodies expected by
//! `fixtures/graphql_schema.json`.

use spikard::testing::test_client_from_app;
use spikard::{App, Method, SchemaConfig};

async fn post_graphql(app: App, query: &str) -> serde_json::Value {
    let client = test_client_from_app(app).expect("test client from app");
    let snapshot = client.graphql(query, None, None).await.expect("POST /graphql");
    assert_eq!(snapshot.status, 200);
    snapshot.json().expect("parse json body")
}

#[tokio::test]
async fn test_query_only_schema_returns_seeded_user() {
    let mut app = App::new();
    app.register_graphql_route("/graphql", Method::Post, "query_only", &SchemaConfig::default())
        .expect("register graphql route");

    let body = post_graphql(app, "{ user { id name } }").await;

    assert_eq!(
        body,
        serde_json::json!({
            "data": {
                "user": {
                    "id": "1",
                    "name": "Test User"
                }
            }
        })
    );
}

#[tokio::test]
async fn test_query_mutation_schema_creates_user() {
    let mut app = App::new();
    app.register_graphql_route("/graphql", Method::Post, "query_mutation", &SchemaConfig::default())
        .expect("register graphql route");

    let body = post_graphql(app, r#"mutation { createUser(name: "New") { id name } }"#).await;

    assert_eq!(
        body,
        serde_json::json!({
            "data": {
                "createUser": {
                    "id": "2",
                    "name": "New"
                }
            }
        })
    );
}

#[tokio::test]
async fn test_full_schema_returns_typename() {
    let mut app = App::new();
    app.register_graphql_route("/graphql", Method::Post, "full", &SchemaConfig::default())
        .expect("register graphql route");

    let body = post_graphql(app, "{ __typename }").await;

    assert_eq!(
        body,
        serde_json::json!({
            "data": {
                "__typename": "Query"
            }
        })
    );
}

#[tokio::test]
async fn test_introspection_disabled_rejects_schema_query() {
    let mut app = App::new();
    let mut config = SchemaConfig::default();
    config.set_introspection_enabled(false);
    app.register_graphql_route("/graphql", Method::Post, "query_only", &config)
        .expect("register graphql route");

    let body = post_graphql(app, "{ __schema { types { name } } }").await;

    assert_eq!(
        body,
        serde_json::json!({
            "errors": [
                { "message": "GraphQL introspection is disabled" }
            ]
        })
    );
}

#[tokio::test]
async fn test_depth_limit_exceeded_rejects_deep_query() {
    let mut app = App::new();
    let mut config = SchemaConfig::default();
    config.set_depth_limit(3);
    app.register_graphql_route("/graphql", Method::Post, "query_only", &config)
        .expect("register graphql route");

    let body = post_graphql(app, "{ user { posts { comments { id } } } }").await;

    assert_eq!(
        body,
        serde_json::json!({
            "errors": [
                { "message": "Query depth limit exceeded" }
            ]
        })
    );
}

#[tokio::test]
async fn test_complexity_limit_exceeded_rejects_expensive_query() {
    let mut app = App::new();
    let mut config = SchemaConfig::default();
    config.set_complexity_limit(1);
    app.register_graphql_route("/graphql", Method::Post, "query_only", &config)
        .expect("register graphql route");

    let body = post_graphql(
        app,
        "{ user { id name email posts { id title comments { id text } } } }",
    )
    .await;

    assert_eq!(
        body,
        serde_json::json!({
            "errors": [
                { "message": "Query complexity limit exceeded" }
            ]
        })
    );
}

#[tokio::test]
async fn test_unknown_schema_type_is_rejected() {
    let mut app = App::new();
    let result = app.register_graphql_route("/graphql", Method::Post, "invalid", &SchemaConfig::default());

    assert!(result.is_err());
}
