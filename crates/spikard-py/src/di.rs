//! Python dependency injection implementations
//!
//! This module provides Python-specific implementations of the Dependency trait,
//! bridging Python values and factories to the Rust DI system.

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::into_future;
use spikard_core::di::{Dependency, ResolvedDependencies};
use spikard_core::request_data::RequestData;
use std::any::Any;
use std::sync::Arc;

/// Python value dependency
///
/// Wraps a Python object as a static dependency value
pub struct PythonValueDependency {
    key: String,
    value: Py<PyAny>,
}

impl PythonValueDependency {
    pub fn new(key: String, value: Py<PyAny>) -> Self {
        Self { key, value }
    }
}

impl Dependency for PythonValueDependency {
    fn depends_on(&self) -> &[String] {
        &[] // Value dependencies have no dependencies
    }

    fn resolve<'a>(
        &'a self,
        _resolved: &'a ResolvedDependencies,
        _request: &'a http::Request<()>,
        _request_data: &'a RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, String>> + Send + 'a>>
    {
        Box::pin(async move {
            // Clone the Python object to return
            Python::with_gil(|py| {
                let value = self.value.clone_ref(py);
                Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
            })
        })
    }

    fn is_singleton(&self) -> bool {
        true // Value dependencies are always singletons
    }

    fn is_cacheable(&self) -> bool {
        true
    }
}

/// Python factory dependency
///
/// Wraps a Python callable as a factory dependency
pub struct PythonFactoryDependency {
    factory: Py<PyAny>,
    depends_on: Vec<String>,
    singleton: bool,
    cacheable: bool,
    is_async: bool,
    is_async_generator: bool,
}

impl PythonFactoryDependency {
    pub fn new(
        factory: Py<PyAny>,
        depends_on: Vec<String>,
        singleton: bool,
        cacheable: bool,
        is_async: bool,
        is_async_generator: bool,
    ) -> Self {
        Self {
            factory,
            depends_on,
            singleton,
            cacheable,
            is_async,
            is_async_generator,
        }
    }
}

impl Dependency for PythonFactoryDependency {
    fn depends_on(&self) -> &[String] {
        &self.depends_on
    }

    fn resolve<'a>(
        &'a self,
        resolved: &'a ResolvedDependencies,
        _request: &'a http::Request<()>,
        _request_data: &'a RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, String>> + Send + 'a>>
    {
        Box::pin(async move {
            Python::with_gil(|py| {
                // Build kwargs from resolved dependencies
                let kwargs = pyo3::types::PyDict::new(py);
                for dep_key in &self.depends_on {
                    if let Some(dep_value) = resolved.get::<Py<PyAny>>(dep_key) {
                        kwargs
                            .set_item(dep_key, dep_value.clone_ref(py))
                            .map_err(|e| format!("Failed to set dependency {}: {}", dep_key, e))?;
                    }
                }

                // Call the factory
                let factory = self.factory.bind(py);

                if self.is_async {
                    // Async factory
                    let coroutine = factory
                        .call((), Some(&kwargs))
                        .map_err(|e| format!("Failed to call async factory: {}", e))?;

                    // Drop GIL and await the coroutine
                    py.allow_threads(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            Python::with_gil(|py| {
                                let coroutine = coroutine.clone_ref(py);

                                // Convert Python coroutine to Rust future
                                let future = into_future(coroutine.bind(py))
                                    .map_err(|e| format!("Failed to convert coroutine to future: {}", e))?;

                                Ok::<_, String>(future)
                            })
                        })
                    })?
                    .await
                    .map_err(|e| format!("Async factory failed: {}", e))
                    .and_then(|result| {
                        Python::with_gil(|py| {
                            if self.is_async_generator {
                                // For generators, call __anext__ to get the first value
                                let aiter = result.bind(py);
                                let first_value = aiter
                                    .call_method0("__anext__")
                                    .map_err(|e| format!("Failed to get first value from generator: {}", e))?;

                                // Convert to future and await
                                let value_future = into_future(first_value)
                                    .map_err(|e| format!("Failed to await generator value: {}", e))?;

                                Ok(value_future)
                            } else {
                                // Regular async function - result is already the value
                                Ok(result)
                            }
                        })
                    })?
                    .await
                    .map(|value| Arc::new(value) as Arc<dyn Any + Send + Sync>)
                    .map_err(|e: PyErr| {
                        Python::with_gil(|py| {
                            e.print(py);
                            format!("Async factory execution failed: {}", e)
                        })
                    })
                } else {
                    // Sync factory
                    let result = factory
                        .call((), Some(&kwargs))
                        .map_err(|e| format!("Failed to call sync factory: {}", e))?;

                    if self.is_async_generator {
                        // This shouldn't happen (sync generator marked as async_generator)
                        return Err("Sync generator not yet supported".to_string());
                    }

                    Ok(Arc::new(result.into()) as Arc<dyn Any + Send + Sync>)
                }
            })
        })
    }

    fn is_singleton(&self) -> bool {
        self.singleton
    }

    fn is_cacheable(&self) -> bool {
        self.cacheable
    }
}
