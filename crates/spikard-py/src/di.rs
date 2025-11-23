//! Python dependency injection implementations
//!
//! This module provides Python-specific implementations of the Dependency trait,
//! bridging Python values and factories to the Rust DI system.

use http::Request;
use pyo3::prelude::*;
use spikard_core::di::{Dependency, ResolvedDependencies};
use spikard_core::request_data::RequestData;
use std::any::Any;
use std::sync::Arc;

// Import the event loop from handler
use crate::handler::PYTHON_EVENT_LOOP;

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
        let resolved_clone = resolved.clone();

        // Extract resolved dependencies now (before async)
        let resolved_deps: Vec<(String, Py<PyAny>)> = Python::with_gil(|py| {
            self.depends_on
                .iter()
                .filter_map(|dep_key| {
                    resolved
                        .get::<Py<PyAny>>(dep_key)
                        .map(|v| (dep_key.clone(), v.clone_ref(py)))
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

                if is_async || is_async_generator {
                    // Async factory or async generator - return coroutine/generator
                    let coroutine = factory_bound.call((), Some(&kwargs))?;
                    Ok(Either::Coroutine(coroutine.unbind()))
                } else {
                    // Sync factory - return result directly
                    let result = factory_bound.call((), Some(&kwargs))?;
                    Ok(Either::Value(result.unbind()))
                }
            })
            .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                message: format!("Failed to call factory: {}", e),
            })?;

            match coroutine_or_result {
                Either::Coroutine(coroutine_py) => {
                    // Handle generator vs regular async
                    if is_async_generator {
                        // For async generators, the result of calling the function is an async generator object
                        // We need to:
                        // 1. Call __anext__ to get the first yielded value
                        // 2. Store the generator object for cleanup later
                        // 3. Register a cleanup task that will close the generator

                        let generator_obj = Python::with_gil(|py| coroutine_py.clone_ref(py));

                        let final_value = tokio::task::spawn_blocking(move || {
                            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                                let asyncio = py.import("asyncio")?;
                                let event_loop = PYTHON_EVENT_LOOP
                                    .get()
                                    .ok_or_else(|| {
                                        pyo3::exceptions::PyRuntimeError::new_err(
                                            "Python event loop not initialized"
                                        )
                                    })?;

                                // coroutine_py is the async generator object
                                let aiter = coroutine_py.bind(py);
                                let first_value_coro = aiter.call_method0("__anext__")?;

                                // Await the __anext__ coroutine
                                let loop_obj = event_loop.bind(py);
                                let future = asyncio.call_method1("run_coroutine_threadsafe", (first_value_coro, loop_obj))?;
                                let result = future.call_method0("result")?;
                                Ok(result.into())
                            })
                        })
                        .await
                        .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Tokio error in async generator: {}", e),
                        })?
                        .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Async generator __anext__ failed: {}", e),
                        })?;

                        // Register cleanup task to close the generator
                        let mut resolved_mut = resolved_clone;
                        resolved_mut.add_cleanup_task(Box::new(move || {
                            Box::pin(async move {
                                let _ = tokio::task::spawn_blocking(move || {
                                    Python::with_gil(|py| -> PyResult<()> {
                                        let asyncio = py.import("asyncio")?;
                                        let event_loop = PYTHON_EVENT_LOOP
                                            .get()
                                            .ok_or_else(|| {
                                                pyo3::exceptions::PyRuntimeError::new_err(
                                                    "Python event loop not initialized"
                                                )
                                            })?;

                                        // Close the async generator by calling aclose()
                                        let aiter = generator_obj.bind(py);
                                        let close_coro = aiter.call_method0("aclose")?;

                                        // Await the aclose coroutine
                                        let loop_obj = event_loop.bind(py);
                                        let future = asyncio.call_method1("run_coroutine_threadsafe", (close_coro, loop_obj))?;
                                        let _ = future.call_method0("result")?;
                                        Ok(())
                                    })
                                })
                                .await;
                            })
                        }));

                        Ok(Arc::new(final_value) as Arc<dyn Any + Send + Sync>)
                    } else {
                        // Regular async function - await the coroutine
                        let result = tokio::task::spawn_blocking(move || {
                            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                                let asyncio = py.import("asyncio")?;
                                let event_loop = PYTHON_EVENT_LOOP
                                    .get()
                                    .ok_or_else(|| {
                                        pyo3::exceptions::PyRuntimeError::new_err(
                                            "Python event loop not initialized. Call init_python_event_loop() first."
                                        )
                                    })?;

                                let coroutine = coroutine_py.bind(py);
                                let loop_obj = event_loop.bind(py);
                                let future = asyncio.call_method1("run_coroutine_threadsafe", (coroutine, loop_obj))?;

                                // Wait for the result
                                let result = future.call_method0("result")?;
                                Ok(result.into())
                            })
                        })
                        .await
                        .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Tokio error running async factory: {}", e),
                        })?
                        .map_err(|e| {
                            Python::with_gil(|py| {
                                e.print(py);
                            });
                            spikard_core::di::DependencyError::ResolutionFailed {
                                message: format!("Async factory failed: {}", e),
                            }
                        })?;

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
