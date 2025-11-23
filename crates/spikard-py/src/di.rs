//! Python dependency injection implementations
//!
//! This module provides Python-specific implementations of the Dependency trait,
//! bridging Python values and factories to the Rust DI system.

use http::Request;
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
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, spikard_core::di::DependencyError>>
                + Send
                + '_,
        >,
    > {
        let value = Python::with_gil(|py| self.value.clone_ref(py));
        Box::pin(async move {
            // Clone the Python object to return
            Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        vec![] // Value dependencies have no dependencies
    }

    fn singleton(&self) -> bool {
        true // Value dependencies are always singletons
    }

    fn cacheable(&self) -> bool {
        true
    }
}

/// Python factory dependency
///
/// Wraps a Python callable as a factory dependency
pub struct PythonFactoryDependency {
    key: String,
    factory: Py<PyAny>,
    depends_on: Vec<String>,
    singleton: bool,
    cacheable: bool,
    is_async: bool,
    is_async_generator: bool,
}

impl PythonFactoryDependency {
    pub fn new(
        key: String,
        factory: Py<PyAny>,
        depends_on: Vec<String>,
        singleton: bool,
        cacheable: bool,
        is_async: bool,
        is_async_generator: bool,
    ) -> Self {
        Self {
            key,
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
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, spikard_core::di::DependencyError>>
                + Send
                + '_,
        >,
    > {
        // Clone things we need in the async block
        let factory = Python::with_gil(|py| self.factory.clone_ref(py));
        let is_async = self.is_async;
        let is_async_generator = self.is_async_generator;

        // Extract resolved dependencies now (before async)
        let resolved_deps: Vec<(String, Py<PyAny>)> = Python::with_gil(|py| {
            self.depends_on
                .iter()
                .filter_map(|dep_key| {
                    resolved.get::<Py<PyAny>>(dep_key).map(|v| (dep_key.clone(), v.clone_ref(py)))
                })
                .collect()
        });

        Box::pin(async move {
            // Build kwargs and call factory with GIL
            let coroutine_or_result = Python::with_gil(|py| -> PyResult<Either> {
                let kwargs = pyo3::types::PyDict::new(py);
                for (dep_key, dep_value) in &resolved_deps {
                    kwargs.set_item(dep_key, dep_value.bind(py))?;
                }

                let factory_bound = factory.bind(py);

                if is_async {
                    // Async factory - return coroutine
                    let coroutine = factory_bound.call((), Some(&kwargs))?;
                    Ok(Either::Coroutine(coroutine.unbind()))
                } else {
                    // Sync factory - return result directly
                    let result = factory_bound.call((), Some(&kwargs))?;
                    Ok(Either::Value(result.unbind()))
                }
            }).map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                message: format!("Failed to call factory: {}", e),
            })?;

            match coroutine_or_result {
                Either::Coroutine(coroutine_py) => {
                    // Async path: await the coroutine
                    let result = Python::with_gil(|py| {
                        let coroutine = coroutine_py.bind(py).clone();
                        into_future(coroutine)
                    }).map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                        message: format!("Failed to convert coroutine to future: {}", e),
                    })?.await.map_err(|e| {
                        Python::with_gil(|py| {
                            e.print(py);
                        });
                        spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Async factory failed: {}", e),
                        }
                    })?;

                    // Handle generator vs regular async
                    if is_async_generator {
                        // For generators, call __anext__ to get the first value
                        let value = Python::with_gil(|py| {
                            let aiter = result.bind(py);
                            let first_value = aiter.call_method0("__anext__")?;
                            Ok::<_, PyErr>(first_value.unbind())
                        }).map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Failed to get first value from generator: {}", e),
                        })?;

                        // Await the generator value
                        let final_value = Python::with_gil(|py| {
                            let val = value.bind(py).clone();
                            into_future(val)
                        }).map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Failed to await generator value: {}", e),
                        })?.await.map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Generator value await failed: {}", e),
                        })?;

                        Ok(Arc::new(final_value) as Arc<dyn Any + Send + Sync>)
                    } else {
                        // Regular async function - result is already the value
                        Ok(Arc::new(result) as Arc<dyn Any + Send + Sync>)
                    }
                }
                Either::Value(value) => {
                    // Sync path - already have the value
                    Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
                }
            }
        })
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        self.depends_on.clone()
    }

    fn singleton(&self) -> bool {
        self.singleton
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }
}

// Helper enum to avoid returning Option
enum Either {
    Coroutine(Py<PyAny>),
    Value(Py<PyAny>),
}
