//! Elixir atoms used in the Spikard NIF bindings.
//!
//! This module defines all atoms that are used for communication between Elixir
//! and Rust code. Atoms are cached for efficient reuse.
//!
//! # Atom Categories
//!
//! - **Standard Elixir atoms**: `ok`, `error`, `nil`, `true_`, `false_`
//! - **HTTP method atoms**: `get`, `post`, `put`, `patch`, `delete`, `head`, `options`
//! - **Response field atoms**: `status`, `headers`, `body`
//! - **Request field atoms**: `path_params`, `query_params`, `cookies`, `method`, `path`
//! - **Error reason atoms**: `not_implemented`, `server_error`, `invalid_config`, `route_not_found`, `handler_error`, `nif_error`, `validation_error`

rustler::atoms! {
    // Standard Elixir atoms
    ok,
    error,
    nil,
    // Map Rust identifiers to actual Elixir atom names (without underscores)
    true_ = "true",
    false_ = "false",
    stopped,

    // HTTP method atoms
    get,
    post,
    put,
    patch,
    delete,
    head,
    options,

    // Response field atoms
    status,
    headers,
    body,

    // Request field atoms
    path_params,
    query_params,
    cookies,
    method,
    path,

    // Server info atoms
    host,
    port,

    // Error reason atoms
    not_implemented,
    server_error,
    invalid_config,
    invalid_port,
    invalid_routes_json,
    no_routes,
    route_creation_failed,
    router_build_failed,
    invalid_socket_address,
    runtime_error,
    route_not_found,
    handler_error,
    nif_error,
    validation_error,

    // Lifecycle hook result atoms
    continue_ = "continue",
    short_circuit = "short_circuit",
}
