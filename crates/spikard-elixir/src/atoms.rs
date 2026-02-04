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
    true_,
    false_,
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
    route_not_found,
    handler_error,
    nif_error,
    validation_error,
}
