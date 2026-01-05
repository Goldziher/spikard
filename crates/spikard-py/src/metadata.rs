//! Python FFI bindings for metadata extraction
//!
//! Provides Python access to Rust-based metadata extraction and validation functions.

use pyo3::prelude::*;
use spikard_core::metadata::{ParameterMetadata, extract_path_parameters, merge_parameters, parse_parameter_schema};

use crate::conversion::python_to_json;

/// `PyO3` wrapper for ParameterMetadata
#[pyclass]
#[derive(Clone, Debug)]
pub struct PyParameterMetadata {
    inner: ParameterMetadata,
}

#[pymethods]
impl PyParameterMetadata {
    #[new]
    fn new(
        name: String,
        source: String,
        schema_type: Option<String>,
        required: bool,
        schema: Option<&Bound<'_, PyAny>>,
        py: Python<'_>,
    ) -> PyResult<Self> {
        let schema_value = schema.map(|s| python_to_json(py, s)).transpose()?;

        Ok(PyParameterMetadata {
            inner: ParameterMetadata {
                name,
                source: source.parse().map_err(|e: String| {
                    pyo3::exceptions::PyValueError::new_err(format!("Invalid parameter source: {e}"))
                })?,
                schema_type,
                required,
                schema: schema_value,
            },
        })
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[getter]
    fn source(&self) -> String {
        self.inner.source.to_string()
    }

    #[getter]
    fn schema_type(&self) -> Option<String> {
        self.inner.schema_type.clone()
    }

    #[getter]
    fn required(&self) -> bool {
        self.inner.required
    }

    #[getter]
    fn schema(&self, py: Python<'_>) -> PyResult<Option<Bound<'_, PyAny>>> {
        use crate::conversion::json_to_python;
        self.inner.schema.as_ref().map(|v| json_to_python(py, v)).transpose()
    }

    fn __repr__(&self) -> String {
        format!(
            "PyParameterMetadata(name='{}', source='{}', required={})",
            self.inner.name, self.inner.source, self.inner.required
        )
    }
}

/// Extract path parameters from a URL pattern
///
/// Args:
///     path: URL pattern like "/users/{user_id}/posts/{post_id}"
///
/// Returns:
///     List of ParameterMetadata objects for path parameters
#[pyfunction]
fn extract_path_params_py(path: &str) -> PyResult<Vec<PyParameterMetadata>> {
    let params = extract_path_parameters(path);
    Ok(params.into_iter().map(|p| PyParameterMetadata { inner: p }).collect())
}

/// Parse parameter schema from JSON Schema
///
/// Args:
///     schema_dict: JSON Schema object with "properties" and "required" keys
///
/// Returns:
///     List of ParameterMetadata objects
#[pyfunction]
fn parse_parameter_schema_py(py: Python<'_>, schema_dict: &Bound<'_, PyAny>) -> PyResult<Vec<PyParameterMetadata>> {
    let schema_value = python_to_json(py, schema_dict)?;
    let params = parse_parameter_schema(&schema_value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse schema: {e}")))?;

    Ok(params.into_iter().map(|p| PyParameterMetadata { inner: p }).collect())
}

/// Merge path parameters with schema parameters
///
/// Args:
///     path: URL pattern for extracting path parameters
///     schema_dict: Optional parameter schema dict
///
/// Returns:
///     Merged list of ParameterMetadata objects
#[pyfunction]
fn merge_params_py(
    py: Python<'_>,
    path: &str,
    schema_dict: Option<&Bound<'_, PyAny>>,
) -> PyResult<Vec<PyParameterMetadata>> {
    let path_params = extract_path_parameters(path);

    let schema_value = schema_dict.map(|s| python_to_json(py, s)).transpose()?;

    let merged = merge_parameters(path_params, schema_value.as_ref())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to merge parameters: {e}")))?;

    Ok(merged.into_iter().map(|p| PyParameterMetadata { inner: p }).collect())
}

/// Register metadata functions with Python module
pub fn register_metadata_functions(m: &Bound<'_, pyo3::types::PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_path_params_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_parameter_schema_py, m)?)?;
    m.add_function(wrap_pyfunction!(merge_params_py, m)?)?;
    m.add_class::<PyParameterMetadata>()?;
    Ok(())
}
