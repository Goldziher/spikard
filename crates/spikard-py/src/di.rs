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
        let value = Python::attach(|py| self.value.clone_ref(py));
        Box::pin(async move { Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>) })
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        vec![]
    }

    fn singleton(&self) -> bool {
        true
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
        let factory = Python::attach(|py| self.factory.clone_ref(py));
        let is_async = self.is_async;
        let is_async_generator = self.is_async_generator;
        let resolved_clone = resolved.clone();

        let resolved_deps: Vec<(String, Py<PyAny>)> = Python::attach(|py| {
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
            let coroutine_or_result = Python::attach(|py| -> PyResult<Either> {
                let kwargs = pyo3::types::PyDict::new(py);
                for (dep_key, dep_value) in &resolved_deps {
                    kwargs.set_item(dep_key, dep_value.bind(py))?;
                }

                let factory_bound = factory.bind(py);

                if is_async || is_async_generator {
                    let coroutine = factory_bound.call((), Some(&kwargs))?;
                    Ok(Either::Coroutine(coroutine.unbind()))
                } else {
                    let result = factory_bound.call((), Some(&kwargs))?;
                    Ok(Either::Value(result.unbind()))
                }
            })
            .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                message: format!("Failed to call factory: {e}"),
            })?;

            match coroutine_or_result {
                Either::Coroutine(coroutine_py) => {
                    if is_async_generator {
                        let generator_obj = Python::attach(|py| coroutine_py.clone_ref(py));

                        let anext_coro = Python::attach(|py| -> PyResult<Py<PyAny>> {
                            let aiter = coroutine_py.bind(py);
                            let first_value_coro = aiter.call_method0("__anext__")?;
                            Ok(first_value_coro.unbind())
                        })
                        .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Failed to call __anext__: {e}"),
                        })?;

                        let final_value =
                            Python::attach(|py| -> Result<Py<PyAny>, spikard_core::di::DependencyError> {
                                let asyncio = py.import("asyncio").map_err(|e| {
                                    spikard_core::di::DependencyError::ResolutionFailed {
                                        message: format!("Failed to import asyncio: {e}"),
                                    }
                                })?;
                                let awaited = asyncio.call_method1("run", (anext_coro.bind(py),)).map_err(|e| {
                                    spikard_core::di::DependencyError::ResolutionFailed {
                                        message: format!("Async generator __anext__ failed: {e}"),
                                    }
                                })?;
                                Ok(awaited.unbind())
                            })?;

                        let resolved_mut = resolved_clone;
                        resolved_mut.add_cleanup_task(Box::new(move || {
                            Box::pin(async move {
                                let _ = Python::attach(|py| -> PyResult<()> {
                                    let aiter = generator_obj.bind(py);
                                    let close_coro = aiter.call_method0("aclose")?;
                                    let asyncio = py.import("asyncio")?;
                                    let _ = asyncio.call_method1("run", (close_coro,))?;
                                    Ok(())
                                });
                            })
                        }));

                        Ok(Arc::new(final_value) as Arc<dyn Any + Send + Sync>)
                    } else {
                        let result = Python::attach(|py| -> Result<Py<PyAny>, spikard_core::di::DependencyError> {
                            let asyncio = py.import("asyncio").map_err(|e| {
                                spikard_core::di::DependencyError::ResolutionFailed {
                                    message: format!("Failed to import asyncio: {e}"),
                                }
                            })?;
                            let awaited = asyncio.call_method1("run", (coroutine_py.bind(py),)).map_err(|e| {
                                spikard_core::di::DependencyError::ResolutionFailed {
                                    message: format!("Async factory failed: {e}"),
                                }
                            })?;
                            Ok(awaited.unbind())
                        })?;

                        Ok(Arc::new(result) as Arc<dyn Any + Send + Sync>)
                    }
                }
                Either::Value(value) => Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>),
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

enum Either {
    Coroutine(Py<PyAny>),
    Value(Py<PyAny>),
}
