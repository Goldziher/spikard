//! Route metadata extraction and building from Ruby definitions.

pub mod route_extraction;

pub use route_extraction::{build_route_metadata, json_to_ruby, ruby_value_to_json};
