//! Dependency injection error types
//!
//! This module defines all error types that can occur during dependency registration
//! and resolution in the DI container.

/// Errors that can occur during dependency injection operations
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::DependencyError;
///
/// let error = DependencyError::NotFound {
///     key: "database".to_string(),
/// };
/// assert_eq!(error.to_string(), "Dependency not found: database");
/// ```
#[derive(thiserror::Error, Debug)]
pub enum DependencyError {
    /// A circular dependency was detected during registration or resolution
    ///
    /// Contains the cycle as a vector of dependency keys in the order they were encountered.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyError;
    ///
    /// let error = DependencyError::CircularDependency {
    ///     cycle: vec!["A".to_string(), "B".to_string(), "A".to_string()],
    /// };
    /// println!("{}", error); // "Circular dependency detected: ["A", "B", "A"]"
    /// ```
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency {
        /// The cycle of dependencies (e.g., `["A", "B", "C", "A"]`)
        cycle: Vec<String>,
    },

    /// A requested dependency was not found in the container
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyError;
    ///
    /// let error = DependencyError::NotFound {
    ///     key: "cache".to_string(),
    /// };
    /// assert!(error.to_string().contains("cache"));
    /// ```
    #[error("Dependency not found: {key}")]
    NotFound {
        /// The key of the missing dependency
        key: String,
    },

    /// Type mismatch when attempting to downcast a dependency to a concrete type
    ///
    /// This occurs when calling `ResolvedDependencies::get<T>()` with the wrong type.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyError;
    ///
    /// let error = DependencyError::TypeMismatch {
    ///     key: "config".to_string(),
    /// };
    /// println!("{}", error); // "Type mismatch for dependency: config"
    /// ```
    #[error("Type mismatch for dependency: {key}")]
    TypeMismatch {
        /// The key of the dependency with the type mismatch
        key: String,
    },

    /// A dependency failed to resolve
    ///
    /// This is a catch-all for any errors that occur during the resolution process,
    /// such as factory function failures or async resolution errors.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyError;
    ///
    /// let error = DependencyError::ResolutionFailed {
    ///     message: "Database connection failed".to_string(),
    /// };
    /// assert!(error.to_string().contains("Database connection"));
    /// ```
    #[error("Resolution failed: {message}")]
    ResolutionFailed {
        /// Description of the resolution failure
        message: String,
    },

    /// A duplicate dependency key was registered
    ///
    /// The DI container does not allow multiple dependencies with the same key.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyError;
    ///
    /// let error = DependencyError::DuplicateKey {
    ///     key: "logger".to_string(),
    /// };
    /// println!("{}", error); // "Duplicate dependency key: logger"
    /// ```
    #[error("Duplicate dependency key: {key}")]
    DuplicateKey {
        /// The duplicate key
        key: String,
    },
}
