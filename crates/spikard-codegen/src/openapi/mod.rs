//! `OpenAPI` 3.1 generation and manipulation

pub mod from_fixtures;
pub mod spec;

pub use from_fixtures::{
    Fixture, FixtureStreamChunk, FixtureStreaming, OpenApiOptions, fixtures_to_openapi, load_fixtures_from_dir,
};
pub use spec::{OpenApiSpec, Operation, PathItem, Response, Schema, SchemaObject};
