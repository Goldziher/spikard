//! Common test utilities for spikard-http tests
//!
//! This module provides reusable test fixtures and mock handlers that implement
//! the Handler trait for testing various scenarios without requiring language bindings.
//!
//! # Examples
//!
//! ```ignore
//! use common::handlers::SuccessHandler;
//! use common::test_builders::{HandlerBuilder, RequestBuilder};
//! use spikard_http::Handler;
//!
//! let handler = SuccessHandler;
//! let fluent_handler = HandlerBuilder::new().status(200).build();
//! // Use in tests...
//! ```

#![allow(dead_code)]

pub mod grpc_helpers;
pub mod handlers;
pub mod test_builders;

#[allow(unused_imports)]
pub use grpc_helpers::{
    EchoMockHandler, ErrorMockHandler, GrpcTestClient, GrpcTestServer, MockGrpcHandler, ProtobufMessageBuilder,
    add_auth_metadata, add_metadata_header, assert_grpc_response, assert_grpc_status, create_grpc_test_client,
    create_test_metadata, create_test_metadata_with_headers, send_unary_request,
};
#[allow(unused_imports)]
pub use handlers::{EchoHandler, ErrorHandler, JsonHandler, PanicHandler, SuccessHandler};
#[allow(unused_imports)]
pub use test_builders::{HandlerBuilder, RequestBuilder, assert_status, load_fixture, parse_json_body};
