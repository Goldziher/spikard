//! Dynamic-SDL GraphQL executor.
//!
//! The [`GraphQLExecutor`](crate::executor::GraphQLExecutor) in [`crate::executor`] requires a
//! statically-typed `async-graphql` schema (concrete `Query`/`Mutation`/`Subscription` Rust
//! types), which is what Spikard's built-in test schemas use. This module instead builds an
//! [`async_graphql::dynamic::Schema`] at runtime from an arbitrary GraphQL SDL string, which is
//! the shape needed to drive fixture-defined schemas (see `fixtures/graphql_operations.json`)
//! without generating a Rust type per fixture.
//!
//! # Resolver-data encoding
//!
//! Fixtures do not implement resolvers in Rust. Instead each fixture supplies a
//! `response_data` JSON tree shaped exactly like the successful `data` payload the query is
//! expected to produce, for example:
//!
//! ```json
//! {
//!   "user": { "id": "user-42", "name": "Alice" }
//! }
//! ```
//!
//! Every dynamic field resolver is *generic*: it looks up its own field name inside the JSON
//! object it was handed (the root `response_data` for top-level query/mutation fields, or the
//! parent field's JSON sub-object for nested fields) and returns that value directly. Because
//! argument-dependent values (like `user(id: "user-42")`) are already baked into
//! `response_data` at the correct field path, this name-based navigation reproduces
//! argument-shaped results without the executor ever inspecting arguments.
//!
//! Field-level errors are encoded with a parallel `field_errors` list of `(path, message)`
//! pairs, where `path` is the dot-separated field path (e.g. `"user"` or `"order.customer"`).
//! When a resolver's path matches an entry, it reports that message against the field's own
//! response path (via `Context::set_error_path`/`Context::add_error`) and resolves to `null`
//! instead of reading `response_data`. This matches the GraphQL spec's partial-data shape:
//! sibling fields still resolve normally, and the failed field appears as `null` in `data`
//! alongside a matching entry (with `path`) in `errors`.

use async_graphql::dynamic::{
    Field, FieldFuture, FieldValue, InputObject, InputValue, Object, Schema, SchemaError as DynamicSchemaError, TypeRef,
};
use async_graphql::{Request, Value as GraphQLValue, Variables};
use async_graphql_parser::types::{
    BaseType, ServiceDocument, Type as AstType, TypeDefinition, TypeKind, TypeSystemDefinition,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::error::GraphQLError;

/// A field-level error to inject at a specific response path.
///
/// `path` is the dot-separated sequence of field names from the operation root to the field
/// that should fail, e.g. `"user"` for a top-level field or `"order.customer"` for a nested one.
#[derive(Debug, Clone)]
pub struct FieldErrorSpec {
    /// Dot-separated path to the field that should error.
    pub path: String,
    /// The error message to surface for that field.
    pub message: String,
}

/// Configuration for building and executing a dynamic-SDL schema.
#[derive(Debug, Clone, Default)]
pub struct DynamicSchemaConfig {
    /// Whether introspection queries (`__schema`, `__type`) are permitted.
    pub introspection_enabled: bool,
    /// Maximum query complexity (`None` = unlimited).
    pub max_complexity: Option<usize>,
    /// Maximum query depth (`None` = unlimited).
    pub max_depth: Option<usize>,
    /// Field-level errors to inject at specific response paths.
    pub field_errors: Vec<FieldErrorSpec>,
}

/// Build an `async-graphql` dynamic schema from an SDL string and a `response_data` JSON tree.
///
/// Every object field in the SDL is given a generic resolver that navigates `response_data` (or
/// the current parent JSON node, for nested fields) by field name. See the [module
/// documentation](self) for the full resolver-data encoding.
///
/// # Errors
///
/// Returns [`GraphQLError::SchemaBuildError`] if the SDL cannot be parsed, defines no query root,
/// or otherwise fails `async-graphql`'s dynamic-schema validation (e.g. duplicate field names).
pub fn build_dynamic_schema(
    sdl: &str,
    response_data: JsonValue,
    config: &DynamicSchemaConfig,
) -> Result<Schema, GraphQLError> {
    let document = async_graphql_parser::parse_schema(sdl)
        .map_err(|e| GraphQLError::SchemaBuildError(format!("invalid SDL: {e}")))?;

    let roots = schema_roots(&document);
    let field_errors = index_field_errors(&config.field_errors);

    let mut builder = Schema::build(&roots.query, roots.mutation.as_deref(), roots.subscription.as_deref());

    for definition in &document.definitions {
        let TypeSystemDefinition::Type(positioned_type) = definition else {
            continue;
        };
        builder = register_type(
            builder,
            &positioned_type.node,
            roots.subscription.as_deref(),
            &field_errors,
        )?;
    }

    if !config.introspection_enabled {
        builder = builder.disable_introspection();
    }
    if let Some(max_complexity) = config.max_complexity {
        builder = builder.limit_complexity(max_complexity);
    }
    if let Some(max_depth) = config.max_depth {
        builder = builder.limit_depth(max_depth);
    }

    let schema = builder
        .data(RootData(response_data))
        .finish()
        .map_err(|e: DynamicSchemaError| GraphQLError::SchemaBuildError(e.to_string()))?;

    Ok(schema)
}

/// Root data stored in the schema's context, holding the fixture's `response_data` tree.
struct RootData(JsonValue);

/// The names of a schema's query/mutation/subscription root types, parsed from an
/// `SDLSchemaDefinition` if present, falling back to the conventional `Query`/`Mutation`/
/// `Subscription` type names otherwise.
struct SchemaRoots {
    query: String,
    mutation: Option<String>,
    subscription: Option<String>,
}

/// Determine the schema's root operation type names.
///
/// Looks for an explicit `schema { query: ... }` definition first; otherwise falls back to the
/// conventional `Query`/`Mutation`/`Subscription` type names, only including a root if a type
/// with that name is actually defined in the document.
fn schema_roots(document: &ServiceDocument) -> SchemaRoots {
    for definition in &document.definitions {
        if let TypeSystemDefinition::Schema(positioned) = definition {
            let schema_def = &positioned.node;
            return SchemaRoots {
                query: schema_def
                    .query
                    .as_ref()
                    .map_or_else(|| "Query".to_string(), |n| n.node.to_string()),
                mutation: schema_def.mutation.as_ref().map(|n| n.node.to_string()),
                subscription: schema_def.subscription.as_ref().map(|n| n.node.to_string()),
            };
        }
    }

    let has_type = |name: &str| {
        document
            .definitions
            .iter()
            .any(|definition| matches!(definition, TypeSystemDefinition::Type(t) if t.node.name.node.as_str() == name))
    };

    SchemaRoots {
        query: "Query".to_string(),
        mutation: has_type("Mutation").then(|| "Mutation".to_string()),
        subscription: has_type("Subscription").then(|| "Subscription".to_string()),
    }
}

/// Build a lookup from dot-separated field path to error message for O(1) resolver checks.
fn index_field_errors(specs: &[FieldErrorSpec]) -> HashMap<String, String> {
    specs
        .iter()
        .map(|spec| (spec.path.clone(), spec.message.clone()))
        .collect()
}

/// Register a single SDL type definition (object or input object) on the dynamic schema builder.
///
/// Scalar, enum, interface, and union kinds are intentionally unsupported: the fixture corpus
/// this executor serves only exercises object and input-object types (see
/// `fixtures/graphql_operations.json`), and adding speculative support for kinds with no
/// fixture coverage would be unverifiable dead code.
fn register_type(
    builder: async_graphql::dynamic::SchemaBuilder,
    type_def: &TypeDefinition,
    subscription_root_name: Option<&str>,
    field_errors: &HashMap<String, String>,
) -> Result<async_graphql::dynamic::SchemaBuilder, GraphQLError> {
    let type_name = type_def.name.node.to_string();
    let is_subscription_root = subscription_root_name == Some(type_name.as_str());

    match &type_def.kind {
        TypeKind::Object(object_type) if is_subscription_root => {
            let mut subscription = async_graphql::dynamic::Subscription::new(&type_name);
            for field in &object_type.fields {
                subscription = subscription.field(build_subscription_field(&field.node));
            }
            Ok(builder.register(subscription))
        }
        TypeKind::Object(object_type) => {
            let mut object = Object::new(&type_name);
            for field in &object_type.fields {
                object = object.field(build_field(&type_name, &field.node, field_errors));
            }
            Ok(builder.register(object))
        }
        TypeKind::InputObject(input_object_type) => {
            let mut input = InputObject::new(&type_name);
            for field in &input_object_type.fields {
                let ty = ast_type_to_type_ref(&field.node.ty.node);
                input = input.field(InputValue::new(field.node.name.node.to_string(), ty));
            }
            Ok(builder.register(input))
        }
        TypeKind::Scalar | TypeKind::Enum(_) | TypeKind::Interface(_) | TypeKind::Union(_) => {
            Err(GraphQLError::SchemaBuildError(format!(
                "dynamic-SDL executor only supports object and input types; unsupported kind for type \"{type_name}\""
            )))
        }
    }
}

/// Build a generic subscription field resolver for the subscription root.
///
/// The resolver reads its own field name out of the root `response_data` tree and emits it as a
/// single-item stream. `async-graphql`'s dynamic `Schema::execute` (the synchronous
/// request/response transport this executor uses) rejects subscription operations outright with
/// "Subscriptions are not supported on this transport", so this resolver only takes effect via
/// `execute_stream` (WebSocket transport); see the module-level caveat in
/// [`build_dynamic_schema`] for what this means for POST-based subscription fixtures.
fn build_subscription_field(
    field_def: &async_graphql_parser::types::FieldDefinition,
) -> async_graphql::dynamic::SubscriptionField {
    let field_name = field_def.name.node.to_string();
    let type_ref = ast_type_to_type_ref(&field_def.ty.node);

    let mut field = async_graphql::dynamic::SubscriptionField::new(field_name.clone(), type_ref, move |ctx| {
        let field_name = field_name.clone();
        async_graphql::dynamic::SubscriptionFieldFuture::new(async move {
            let root: &RootData = ctx.ctx.data_unchecked();
            let node = root.0.get(&field_name).cloned();
            Ok(futures::stream::once(async move {
                json_to_field_value(node.as_ref()).map(|value| value.unwrap_or(FieldValue::NULL))
            }))
        })
    });

    for argument in &field_def.arguments {
        let arg_ty = ast_type_to_type_ref(&argument.node.ty.node);
        field = field.argument(InputValue::new(argument.node.name.node.to_string(), arg_ty));
    }

    field
}

/// Build a generic field resolver for `parent_type.field_name`.
///
/// The resolver navigates the JSON tree by field name: at the schema roots (Query/Mutation/
/// Subscription) it reads from the root `response_data`; for nested object fields it reads from
/// the parent field's own JSON sub-object, threaded through `FieldValue::owned_any`.
fn build_field(
    parent_type: &str,
    field_def: &async_graphql_parser::types::FieldDefinition,
    field_errors: &HashMap<String, String>,
) -> Field {
    let field_name = field_def.name.node.to_string();
    let type_ref = ast_type_to_type_ref(&field_def.ty.node);
    let is_root = matches!(parent_type, "Query" | "Mutation" | "Subscription");
    let field_errors = field_errors.clone();

    let mut field = Field::new(field_name.clone(), type_ref, move |ctx| {
        let field_name = field_name.clone();
        let field_errors = field_errors.clone();
        FieldFuture::new(async move {
            let path = resolver_path(&ctx);
            if let Some(message) = field_errors.get(&path) {
                let server_error = async_graphql::Error::new(message.clone()).into_server_error(ctx.ctx.item.pos);
                ctx.ctx.add_error(ctx.ctx.set_error_path(server_error));
                return Ok(None);
            }

            let node = if is_root {
                let root: &RootData = ctx.ctx.data_unchecked();
                root.0.get(&field_name)
            } else {
                let parent_json: &JsonValue = ctx
                    .parent_value
                    .downcast_ref::<JsonValue>()
                    .ok_or_else(|| async_graphql::Error::new("internal: missing parent JSON node"))?;
                parent_json.get(&field_name)
            };

            json_to_field_value(node)
        })
    });

    for argument in &field_def.arguments {
        let arg_ty = ast_type_to_type_ref(&argument.node.ty.node);
        field = field.argument(InputValue::new(argument.node.name.node.to_string(), arg_ty));
    }

    field
}

/// Reconstruct the dot-separated response path of the field currently being resolved.
fn resolver_path(ctx: &async_graphql::dynamic::ResolverContext<'_>) -> String {
    let mut segments: Vec<String> = ctx
        .ctx
        .path_node
        .map(async_graphql::QueryPathNode::to_string_vec)
        .unwrap_or_default();
    if segments.is_empty() {
        segments.push(ctx.ctx.field().name().to_string());
    }
    segments.join(".")
}

/// Convert a JSON node into the `FieldValue` async-graphql expects from a resolver.
///
/// - `null`/missing nodes resolve to `Ok(None)` (a null result).
/// - Scalars (`String`, `Number`, `Bool`) convert directly to `Value`.
/// - Arrays convert to a `FieldValue::list`, recursing per element.
/// - Objects are kept as an owned `serde_json::Value` so nested resolvers can navigate them by
///   field name via `ctx.parent_value`.
fn json_to_field_value<'a>(node: Option<&JsonValue>) -> async_graphql::Result<Option<FieldValue<'a>>> {
    match node {
        None | Some(JsonValue::Null) => Ok(None),
        Some(JsonValue::Object(_)) => Ok(Some(FieldValue::owned_any(node.cloned().unwrap_or(JsonValue::Null)))),
        Some(JsonValue::Array(items)) => {
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                values.push(json_to_field_value(Some(item))?.map_or(FieldValue::NULL, |value| value));
            }
            Ok(Some(FieldValue::list(values)))
        }
        Some(scalar) => {
            let value = GraphQLValue::from_json(scalar.clone())
                .map_err(|e| async_graphql::Error::new(format!("internal: invalid scalar value: {e}")))?;
            Ok(Some(FieldValue::value(value)))
        }
    }
}

/// Convert an SDL AST type (`Type { base, nullable }`) to a dynamic `TypeRef`.
fn ast_type_to_type_ref(ty: &AstType) -> TypeRef {
    let named = base_type_to_type_ref(&ty.base);
    if ty.nullable {
        named
    } else {
        TypeRef::NonNull(Box::new(named))
    }
}

/// Convert an SDL AST base type (`Named` or `List`) to a dynamic `TypeRef`, ignoring
/// nullability (handled by the caller).
fn base_type_to_type_ref(base: &BaseType) -> TypeRef {
    match base {
        BaseType::Named(name) => TypeRef::Named(name.to_string().into()),
        BaseType::List(inner) => TypeRef::List(Box::new(ast_type_to_type_ref(inner))),
    }
}

/// Execute a query/mutation/subscription document against a dynamically-built schema.
///
/// Mirrors [`crate::executor::GraphQLExecutor::execute`]'s response shape: the returned JSON is
/// the full GraphQL-spec response object (`{"data": ..., "errors": [...]}`), ready to serialize
/// directly into an HTTP response body.
///
/// # Errors
///
/// Returns [`GraphQLError::ValidationError`] for an empty query string, or
/// [`GraphQLError::SerializationError`] if the response cannot be converted to JSON. Errors
/// arising from query execution itself (syntax errors, validation failures, complexity/depth
/// limits, field-level errors) are not raised here — they are reported inside the returned JSON
/// value's `errors` array, per the GraphQL spec.
pub async fn execute_dynamic(
    schema: &Schema,
    query: &str,
    variables: Option<&JsonValue>,
    operation_name: Option<&str>,
) -> Result<JsonValue, GraphQLError> {
    if query.trim().is_empty() {
        return Err(GraphQLError::ValidationError(
            "Query string cannot be empty".to_string(),
        ));
    }

    let mut request = Request::new(query);
    if let Some(vars) = variables {
        request = request.variables(Variables::from_json(vars.clone()));
    }
    if let Some(name) = operation_name {
        request = request.operation_name(name);
    }

    let response = schema.execute(request).await;

    serde_json::to_value(response)
        .map_err(|error| GraphQLError::SerializationError(format!("Failed to serialize GraphQL response: {error}")))
}

/// HTTP handler that serves a single pre-built dynamic-SDL schema.
///
/// This is the dynamic-schema counterpart to [`crate::handler::GraphQLHandler`]: where that
/// type is generic over concrete Rust `Query`/`Mutation`/`Subscription` types, this handler
/// wraps an already-built [`Schema`] and is therefore monomorphic, letting it cross Spikard's
/// `Arc<dyn Handler>` FFI-facing boundary directly.
#[derive(Debug, Clone)]
pub struct DynamicGraphQLHandler {
    schema: std::sync::Arc<Schema>,
}

impl DynamicGraphQLHandler {
    /// Wrap a pre-built dynamic schema for HTTP handling.
    #[must_use]
    pub fn new(schema: Schema) -> Self {
        Self {
            schema: std::sync::Arc::new(schema),
        }
    }

    /// Parse and execute a GraphQL HTTP request body against the wrapped schema.
    ///
    /// # Errors
    ///
    /// Returns [`GraphQLError::RequestHandlingError`] if the request body is not valid JSON or
    /// is missing the required `query` field. Errors arising from executing the GraphQL document
    /// itself are reported inside the returned JSON's `errors` array rather than as an `Err`.
    pub async fn handle(&self, raw_body: &[u8]) -> Result<JsonValue, GraphQLError> {
        let payload: crate::handler::GraphQLRequestPayload = serde_json::from_slice(raw_body)
            .map_err(|e| GraphQLError::RequestHandlingError(format!("Failed to parse GraphQL request: {e}")))?;

        execute_dynamic(
            &self.schema,
            &payload.query,
            payload.variables.as_ref(),
            payload.operation_name.as_deref(),
        )
        .await
    }
}

impl spikard_http::handler_trait::Handler for DynamicGraphQLHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        request_data: spikard_http::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::handler_trait::HandlerResult> + Send + '_>>
    {
        Box::pin(async move {
            let body_bytes = request_data.raw_body.as_ref().map_or_else(
                || serde_json::to_vec(&request_data.body).unwrap_or_default(),
                |raw_body| raw_body.to_vec(),
            );

            let result = self.handle(&body_bytes).await;
            let (status, body) = match result {
                Ok(graphql_response) => (axum::http::StatusCode::OK, graphql_response),
                Err(error) => (
                    axum::http::StatusCode::from_u16(error.status_code())
                        .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
                    error.to_graphql_response(),
                ),
            };

            let body_bytes = serde_json::to_vec(&body)
                .unwrap_or_else(|_| b"{\"errors\":[{\"message\":\"Internal server error\"}]}".to_vec());

            axum::http::Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(body_bytes))
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        })
    }

    fn prefers_raw_json_body(&self) -> bool {
        true
    }

    fn wants_headers(&self) -> bool {
        false
    }

    fn wants_cookies(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn config() -> DynamicSchemaConfig {
        DynamicSchemaConfig {
            introspection_enabled: true,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_simple_query_resolves_from_response_data() {
        let schema = build_dynamic_schema("type Query { hello: String! }", json!({"hello": "world"}), &config())
            .expect("schema should build");

        let result = execute_dynamic(&schema, "{ hello }", None, None)
            .await
            .expect("execute");
        assert_eq!(result, json!({"data": {"hello": "world"}}));
    }

    #[tokio::test]
    async fn test_nested_query_navigates_json_tree() {
        let sdl = "type Query { order(id: ID!): Order } type Order { id: ID! customer: Customer } type Customer { name: String! }";
        let response_data = json!({
            "order": {"id": "ORD-1", "customer": {"name": "Bob"}}
        });
        let schema = build_dynamic_schema(sdl, response_data, &config()).expect("schema should build");

        let result = execute_dynamic(&schema, "{ order(id: \"ORD-1\") { id customer { name } } }", None, None)
            .await
            .expect("execute");

        assert_eq!(
            result,
            json!({"data": {"order": {"id": "ORD-1", "customer": {"name": "Bob"}}}})
        );
    }

    #[tokio::test]
    async fn test_field_level_error_reports_path_and_partial_data() {
        let sdl = "type Query { user(id: ID!): User status: String! } type User { id: ID! name: String! }";
        let response_data = json!({"user": null, "status": "ok"});
        let mut cfg = config();
        cfg.field_errors.push(FieldErrorSpec {
            path: "user".to_string(),
            message: "User not found".to_string(),
        });
        let schema = build_dynamic_schema(sdl, response_data, &cfg).expect("schema should build");

        let result = execute_dynamic(&schema, "{ user(id: \"missing-user\") { id name } status }", None, None)
            .await
            .expect("execute");

        assert_eq!(result["data"]["user"], JsonValue::Null);
        assert_eq!(result["data"]["status"], "ok");
        assert_eq!(result["errors"][0]["message"], "User not found");
        assert_eq!(result["errors"][0]["path"], json!(["user"]));
    }

    #[tokio::test]
    async fn test_complexity_limit_is_enforced() {
        let sdl = "type Query { users: [User!]! } type User { id: ID! friends: [User!]! }";
        let mut cfg = config();
        cfg.max_complexity = Some(1);
        let schema = build_dynamic_schema(sdl, json!({"users": []}), &cfg).expect("schema should build");

        let result = execute_dynamic(&schema, "{ users { id friends { id } } }", None, None)
            .await
            .expect("execute");

        assert!(result["errors"].is_array());
        assert!(!result["errors"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_depth_limit_is_enforced() {
        let sdl = "type Query { a: Level1 } type Level1 { b: Level2 } type Level2 { c: String }";
        let mut cfg = config();
        cfg.max_depth = Some(1);
        let schema = build_dynamic_schema(sdl, json!({"a": {"b": {"c": "x"}}}), &cfg).expect("schema should build");

        let result = execute_dynamic(&schema, "{ a { b { c } } }", None, None)
            .await
            .expect("execute");

        assert!(result["errors"].is_array());
        assert!(!result["errors"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_invalid_syntax_returns_error() {
        let schema =
            build_dynamic_schema("type Query { hello: String! }", json!({}), &config()).expect("schema should build");

        let result = execute_dynamic(&schema, "{ hello { this is not valid syntax {{{", None, None)
            .await
            .expect("execute");

        assert!(result["errors"].is_array());
        assert!(!result["errors"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_undefined_field_returns_validation_error() {
        let schema = build_dynamic_schema("type Query { hello: String! }", json!({"hello": "world"}), &config())
            .expect("schema should build");

        let result = execute_dynamic(&schema, "{ hello nonExistentField }", None, None)
            .await
            .expect("execute");

        assert!(result["errors"].is_array());
        assert!(!result["errors"].as_array().unwrap().is_empty());
        assert_eq!(result["data"], JsonValue::Null);
    }

    #[tokio::test]
    async fn test_introspection_disabled_rejects_schema_query() {
        let mut cfg = config();
        cfg.introspection_enabled = false;
        let schema = build_dynamic_schema("type Query { hello: String! }", json!({"hello": "world"}), &cfg)
            .expect("schema should build");

        let result = execute_dynamic(&schema, "{ __schema { queryType { name } } }", None, None)
            .await
            .expect("execute");

        assert!(result["errors"].is_array());
    }

    #[tokio::test]
    async fn test_introspection_query_reports_query_type_name() {
        let schema = build_dynamic_schema("type Query { hello: String! }", json!({"hello": "world"}), &config())
            .expect("schema should build");

        let result = execute_dynamic(&schema, "{ __schema { queryType { name } } }", None, None)
            .await
            .expect("execute");

        assert_eq!(result["data"]["__schema"]["queryType"]["name"], "Query");
    }

    #[tokio::test]
    async fn test_mutation_echoes_input_variables() {
        let sdl = "type Mutation { createUser(input: CreateUserInput!): User! } input CreateUserInput { name: String! email: String! } type User { id: ID! name: String! email: String! } type Query { _placeholder: Boolean }";
        let response_data = json!({
            "createUser": {"id": "new-user-id", "name": "Carol", "email": "carol@example.com"}
        });
        let schema = build_dynamic_schema(sdl, response_data, &config()).expect("schema should build");

        let variables = json!({"input": {"name": "Carol", "email": "carol@example.com"}});
        let result = execute_dynamic(
            &schema,
            "mutation CreateUser($input: CreateUserInput!) { createUser(input: $input) { id name email } }",
            Some(&variables),
            None,
        )
        .await
        .expect("execute");

        assert_eq!(
            result,
            json!({"data": {"createUser": {"id": "new-user-id", "name": "Carol", "email": "carol@example.com"}}})
        );
    }

    #[tokio::test]
    async fn test_operation_name_selects_named_operation() {
        let schema = build_dynamic_schema(
            "type Query { ping: String! version: String! }",
            json!({"ping": "pong", "version": "1.0.0"}),
            &config(),
        )
        .expect("schema should build");

        let result = execute_dynamic(
            &schema,
            "query Ping { ping } query Version { version }",
            None,
            Some("Version"),
        )
        .await
        .expect("execute");

        assert_eq!(result, json!({"data": {"version": "1.0.0"}}));
    }

    #[tokio::test]
    async fn test_empty_query_is_rejected() {
        let schema =
            build_dynamic_schema("type Query { hello: String! }", json!({}), &config()).expect("schema should build");

        let result = execute_dynamic(&schema, "", None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unsupported_scalar_kind_is_rejected() {
        let result = build_dynamic_schema(
            "scalar DateTime type Query { at: DateTime! }",
            json!({"at": "2024-01-01"}),
            &config(),
        );
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_sdl_is_rejected() {
        let result = build_dynamic_schema("not valid sdl {{{", json!({}), &config());
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_field_navigates_each_element() {
        let sdl = "type Query { order(id: ID!): Order } type Order { items: [OrderItem!]! } type OrderItem { sku: String! quantity: Int! }";
        let response_data = json!({
            "order": {"items": [{"sku": "WIDGET-A", "quantity": 2}]}
        });
        let schema = build_dynamic_schema(sdl, response_data, &config()).expect("schema should build");

        let result = execute_dynamic(
            &schema,
            "{ order(id: \"ORD-1\") { items { sku quantity } } }",
            None,
            None,
        )
        .await
        .expect("execute");

        assert_eq!(
            result,
            json!({"data": {"order": {"items": [{"sku": "WIDGET-A", "quantity": 2}]}}})
        );
    }
}
