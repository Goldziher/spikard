//! Ensure documented Rust DI snippet stays compiling.

#[cfg(feature = "di")]
doc_comment::doctest!("../../../docs/snippets/rust/dependency_injection.md");
