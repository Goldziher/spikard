//! Spikard core library
//!
//! This is the main library crate for spikard.

#![warn(missing_docs)]

use thiserror::Error;

/// Main error type for spikard
#[derive(Error, Debug)]
pub enum SpikardError {
    /// Generic error variant
    #[error("Spikard error: {0}")]
    Generic(String),
}

/// Result type alias for spikard operations
pub type Result<T> = std::result::Result<T, SpikardError>;

/// Main entry point for spikard functionality
pub fn process() -> Result<()> {
    // TODO: Implement core functionality
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert!(process().is_ok());
    }
}
