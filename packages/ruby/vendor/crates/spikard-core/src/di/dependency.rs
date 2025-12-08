//! Core dependency trait
//!
//! This module defines the `Dependency` trait which all dependency implementations
//! must implement to integrate with the DI container.

use super::error::DependencyError;
use super::resolved::ResolvedDependencies;
use crate::request_data::RequestData;
use http::Request;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for all dependency implementations
///
/// This trait defines how dependencies are resolved within the DI container.
/// Implementations can be simple values, factories, or complex async providers.
///
/// # Type Erasure
///
/// Dependencies return `Arc<dyn Any + Send + Sync>` to support storing heterogeneous
/// types in the same container. Use `ResolvedDependencies::get<T>()` for type-safe access.
///
/// # Async Resolution
///
/// All dependencies resolve asynchronously, allowing for I/O operations like
/// database connections or HTTP requests during resolution.
///
/// # Caching Strategies
///
/// - **Singleton**: Resolved once globally, cached forever
/// - **Cacheable**: Resolved once per request, cached for that request
/// - **Factory**: Resolved every time it's requested
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::{Dependency, DependencyError, ResolvedDependencies};
/// use http::Request;
/// use crate::request_data::RequestData;
/// use std::any::Any;
/// use std::sync::Arc;
///
/// struct SimpleDependency {
///     key: String,
///     value: Arc<i32>,
/// }
///
/// impl Dependency for SimpleDependency {
///     fn resolve(
///         &self,
///         _request: &Request<()>,
///         _request_data: &RequestData,
///         _resolved: &ResolvedDependencies,
///     ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>> {
///         let value = self.value.clone();
///         Box::pin(async move {
///             Ok(value as Arc<dyn Any + Send + Sync>)
///         })
///     }
///
///     fn key(&self) -> &str {
///         &self.key
///     }
///
///     fn depends_on(&self) -> Vec<String> {
///         vec![]
///     }
/// }
/// ```
pub trait Dependency: Send + Sync {
    /// Resolve this dependency to a concrete value
    ///
    /// This method is called by the DI container to obtain the actual dependency value.
    /// It can perform async operations, access request data, and retrieve other
    /// already-resolved dependencies.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request being handled
    /// * `request_data` - Extracted request data (params, headers, body, etc.)
    /// * `resolved` - Already-resolved dependencies that this dependency may need
    ///
    /// # Returns
    ///
    /// A future that resolves to either:
    /// - `Ok(Arc<dyn Any + Send + Sync>)` - The resolved dependency value
    /// - `Err(DependencyError)` - An error during resolution
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::{Dependency, DependencyError, ResolvedDependencies};
    /// use http::Request;
    /// use crate::request_data::RequestData;
    /// use std::any::Any;
    /// use std::sync::Arc;
    ///
    /// struct ConfigDependency;
    ///
    /// impl Dependency for ConfigDependency {
    ///     fn resolve(
    ///         &self,
    ///         _request: &Request<()>,
    ///         request_data: &RequestData,
    ///         _resolved: &ResolvedDependencies,
    ///     ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>> {
    ///         // Access request data to determine config
    ///         let env = request_data.headers
    ///             .get("X-Environment")
    ///             .map(|s| s.as_str())
    ///             .unwrap_or("production");
    ///
    ///         let config = format!("Config for {}", env);
    ///         Box::pin(async move {
    ///             Ok(Arc::new(config) as Arc<dyn Any + Send + Sync>)
    ///         })
    ///     }
    ///
    ///     fn key(&self) -> &str {
    ///         "config"
    ///     }
    ///
    ///     fn depends_on(&self) -> Vec<String> {
    ///         vec![]
    ///     }
    /// }
    /// ```
    fn resolve(
        &self,
        request: &Request<()>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> BoxFuture<'_, Result<Arc<dyn Any + Send + Sync>, DependencyError>>;

    /// Get the unique key for this dependency
    ///
    /// This key is used to identify the dependency in the container and
    /// when other dependencies reference it.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::Dependency;
    /// # use spikard_core::di::{DependencyError, ResolvedDependencies};
    /// # use http::Request;
    /// # use crate::request_data::RequestData;
    /// # use std::any::Any;
    /// # use std::sync::Arc;
    ///
    /// # struct MyDependency { key: String }
    /// # impl Dependency for MyDependency {
    /// #     fn resolve(&self, _: &Request<()>, _: &RequestData, _: &ResolvedDependencies)
    /// #         -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>
    /// #     { Box::pin(async { Ok(Arc::new(()) as Arc<dyn Any + Send + Sync>) }) }
    /// fn key(&self) -> &str {
    ///     &self.key
    /// }
    /// #     fn depends_on(&self) -> Vec<String> { vec![] }
    /// # }
    /// ```
    fn key(&self) -> &str;

    /// Get the list of dependency keys that this dependency requires
    ///
    /// These dependencies will be resolved before this one, and will be
    /// available in the `resolved` parameter passed to `resolve()`.
    ///
    /// # Returns
    ///
    /// A vector of dependency keys that must be resolved first
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::Dependency;
    /// # use spikard_core::di::{DependencyError, ResolvedDependencies};
    /// # use http::Request;
    /// # use crate::request_data::RequestData;
    /// # use std::any::Any;
    /// # use std::sync::Arc;
    ///
    /// # struct MyDependency;
    /// # impl Dependency for MyDependency {
    /// #     fn resolve(&self, _: &Request<()>, _: &RequestData, _: &ResolvedDependencies)
    /// #         -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>
    /// #     { Box::pin(async { Ok(Arc::new(()) as Arc<dyn Any + Send + Sync>) }) }
    /// #     fn key(&self) -> &str { "service" }
    /// fn depends_on(&self) -> Vec<String> {
    ///     vec!["database".to_string(), "cache".to_string()]
    /// }
    /// # }
    /// ```
    fn depends_on(&self) -> Vec<String>;

    /// Whether this dependency's value can be cached within a single request
    ///
    /// If `true`, the dependency will be resolved once per request and the
    /// same value will be reused for all handlers in that request.
    ///
    /// If `false`, the dependency will be resolved every time it's requested.
    ///
    /// # Default
    ///
    /// Returns `false` by default (no caching).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::Dependency;
    /// # use spikard_core::di::{DependencyError, ResolvedDependencies};
    /// # use http::Request;
    /// # use crate::request_data::RequestData;
    /// # use std::any::Any;
    /// # use std::sync::Arc;
    ///
    /// # struct MyDependency;
    /// # impl Dependency for MyDependency {
    /// #     fn resolve(&self, _: &Request<()>, _: &RequestData, _: &ResolvedDependencies)
    /// #         -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>
    /// #     { Box::pin(async { Ok(Arc::new(()) as Arc<dyn Any + Send + Sync>) }) }
    /// #     fn key(&self) -> &str { "request_id" }
    /// #     fn depends_on(&self) -> Vec<String> { vec![] }
    /// fn cacheable(&self) -> bool {
    ///     true  // Cache within the request
    /// }
    /// # }
    /// ```
    fn cacheable(&self) -> bool {
        false
    }

    /// Whether this dependency is a singleton (cached globally across all requests)
    ///
    /// If `true`, the dependency will be resolved once globally and the same
    /// value will be reused for all requests.
    ///
    /// If `false`, the dependency will be resolved per-request (if cacheable)
    /// or per-use (if not cacheable).
    ///
    /// # Default
    ///
    /// Returns `false` by default (not a singleton).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::Dependency;
    /// # use spikard_core::di::{DependencyError, ResolvedDependencies};
    /// # use http::Request;
    /// # use crate::request_data::RequestData;
    /// # use std::any::Any;
    /// # use std::sync::Arc;
    ///
    /// # struct MyDependency;
    /// # impl Dependency for MyDependency {
    /// #     fn resolve(&self, _: &Request<()>, _: &RequestData, _: &ResolvedDependencies)
    /// #         -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>
    /// #     { Box::pin(async { Ok(Arc::new(()) as Arc<dyn Any + Send + Sync>) }) }
    /// #     fn key(&self) -> &str { "database_pool" }
    /// #     fn depends_on(&self) -> Vec<String> { vec![] }
    /// fn singleton(&self) -> bool {
    ///     true  // Resolve once globally
    /// }
    /// # }
    /// ```
    fn singleton(&self) -> bool {
        false
    }
}
