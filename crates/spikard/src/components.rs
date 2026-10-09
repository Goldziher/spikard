//! Contracts for downloadable native components.
//!
//! ~keep This module is a public ABI boundary. The `spec` contract's trait path and
//! method signature are recorded in `alef.toml` (`[[crates.component_contracts]]`) and
//! hashed into every signed component artifact, so renaming or reshaping anything here
//! invalidates published components rather than merely changing Rust code. Keep the
//! trait object-safe (`Send + Sync`, `&self`, scalar/`str`/`[u8]` wire types only) and
//! keep the error type exported at the crate root as `spikard::ComponentError` — the
//! generated host proxy constructs it as `spikard::ComponentError::from(message)`.

use core::fmt;

/// Error crossing a component's dynamic-library boundary.
///
/// The generated producer and host proxy carry only the message; this type exists so
/// core-crate callers can return a typed error from a contract method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentError(String);

impl ComponentError {
    /// The underlying message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ComponentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ComponentError {}

impl From<String> for ComponentError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

/// The `spec` contract: turn a specification document into spikard's canonical,
/// normalised form.
///
/// A downloadable component implements this behind its own Cargo feature set; the core
/// crate looks the active implementation up through
/// `alef-component-abi`'s provider registry and falls back to [`OpenApiCompiler`] when
/// no component has been activated.
pub trait SpecCompiler: Send + Sync {
    /// Compile `document` (a spec file's raw bytes) for `format`.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentError`] when `format` is unknown or `document` is malformed.
    fn compile(&self, format: &str, document: &[u8]) -> Result<Vec<u8>, ComponentError>;
}

/// The in-process reference compiler for OpenAPI documents.
///
/// Enabled by the `codegen-openapi` feature, which the downloadable `openapi` component
/// turns on in the producer crate. It parses a document into the typed OpenAPI model and
/// re-serialises it as canonical JSON, so both the linked fallback and the downloaded
/// component produce byte-identical output for the same input.
#[cfg(feature = "codegen-openapi")]
#[derive(Debug, Default)]
pub struct OpenApiCompiler;

#[cfg(feature = "codegen-openapi")]
impl SpecCompiler for OpenApiCompiler {
    fn compile(&self, format: &str, document: &[u8]) -> Result<Vec<u8>, ComponentError> {
        match format {
            "openapi" => {
                let spec: spikard_codegen::openapi::OpenApiSpec = serde_json::from_slice(document)
                    .map_err(|error| ComponentError::from(format!("invalid OpenAPI document: {error}")))?;
                serde_json::to_vec(&spec)
                    .map_err(|error| ComponentError::from(format!("serialising OpenAPI document: {error}")))
            }
            other => Err(ComponentError::from(format!("unsupported spec format `{other}`"))),
        }
    }
}

/// Compile `document` for `format` using whichever [`SpecCompiler`] is active.
///
/// This is the core-crate side of the component boundary: a binding downloads, verifies, and
/// activates a component, which registers a [`SpecCompiler`] provider here; core code that needs
/// a spec compiler calls this and never learns whether the active implementation is a downloaded
/// component or an in-process one.
///
/// # Errors
///
/// Returns [`ComponentError`] when no `spec` component is activated yet, or when the active
/// compiler rejects the document.
#[doc(hidden)]
pub fn compile_spec(format: &str, document: &[u8]) -> Result<Vec<u8>, ComponentError> {
    match alef_component_abi::provider::<dyn SpecCompiler>("spec") {
        Some(compiler) => compiler.compile(format, document),
        None => Err(ComponentError::from(
            "no `spec` component is activated; call the binding's component_activate(\"openapi\") first".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct Upper;

    impl SpecCompiler for Upper {
        fn compile(&self, _format: &str, document: &[u8]) -> Result<Vec<u8>, ComponentError> {
            Ok(document.iter().map(u8::to_ascii_uppercase).collect())
        }
    }

    #[test]
    fn compile_spec_dispatches_to_the_registered_component() {
        alef_component_abi::register_provider::<dyn SpecCompiler>("spec", Arc::new(Upper));
        let compiled = compile_spec("openapi", b"hello").expect("registered provider must be used");
        assert_eq!(compiled, b"HELLO");
    }
}
