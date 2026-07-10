#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(warnings)]

pub mod app;
pub mod cli;
pub mod codegen;
pub mod init;
#[cfg(feature = "mcp")]
pub mod mcp;
