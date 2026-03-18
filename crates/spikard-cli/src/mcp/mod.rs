//! Model Context Protocol server for Spikard's codegen-first workflows.

mod errors;
mod params;
mod server;

pub use server::{SpikardMcp, start_mcp_server};

#[cfg(feature = "mcp-http")]
pub use server::start_mcp_server_http;
