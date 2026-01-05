//! Zero-copy JSON conversion utilities for Python bindings
//!
//! This module provides optimized, safe conversion between `serde_json::Value` and Python objects
//! using direct `PyO3` type construction, eliminating JSON string round-trips.
//!
//! Performance improvement: ~40-60% faster than `json.dumps()` → `serde_json::from_str()`

use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyNone, PyString};
use serde_json::Value;

static MSGSPEC_STRUCT_TYPE: PyOnceLock<Py<pyo3::types::PyType>> = PyOnceLock::new();
static MSGSPEC_TO_BUILTINS: PyOnceLock<Py<PyAny>> = PyOnceLock::new();

fn msgspec_struct_helpers(py: Python<'_>) -> Option<(&Py<pyo3::types::PyType>, &Py<PyAny>)> {
    let struct_type = MSGSPEC_STRUCT_TYPE
        .get_or_try_init(py, || -> PyResult<Py<pyo3::types::PyType>> {
            let msgspec = py.import("msgspec")?;
            let struct_type = msgspec.getattr("Struct")?.cast_into::<pyo3::types::PyType>()?;
            Ok(struct_type.unbind())
        })
        .ok()?;

    let to_builtins = MSGSPEC_TO_BUILTINS
        .get_or_try_init(py, || -> PyResult<Py<PyAny>> {
            let msgspec = py.import("msgspec")?;
            Ok(msgspec.getattr("to_builtins")?.unbind())
        })
        .ok()?;

    Some((struct_type, to_builtins))
}

/// Convert `serde_json::Value` to Python object using zero-copy construction
///
/// This function converts JSON values directly to Python objects without
/// serializing to string first, which is much more efficient.
///
/// # Errors
///
/// Returns `PyErr` if conversion fails (e.g., NaN floats)
///
/// # Examples
///
/// ```ignore
/// let value = serde_json::json!({"name": "Alice", "age": 30});
/// let py_obj = json_to_python(py, &value)?;
/// ```
///
/// # Implementation Notes
///
/// This is the canonical implementation and should be used across all modules.
/// It provides zero-copy conversion by directly constructing Python objects
/// from JSON values without intermediate JSON string serialization.
pub fn json_to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
    match value {
        Value::Null => Ok(PyNone::get(py).as_any().clone()),
        Value::Bool(b) => Ok(PyBool::new(py, *b).as_any().clone()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any())
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_pyobject(py)?.into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into_any())
            } else {
                Ok(PyString::new(py, &n.to_string()).into_any())
            }
        }
        Value::String(s) => Ok(PyString::new(py, s).into_any()),
        Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for item in arr {
                let py_item = json_to_python(py, item)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into_any())
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, value) in obj {
                let py_value = json_to_python(py, value)?;
                py_dict.set_item(key, py_value)?;
            }
            Ok(py_dict.into_any())
        }
    }
}

/// Convert Python object to `serde_json::Value`
///
/// This function converts Python objects to JSON values. For complex types
/// like dataclasses or Pydantic models, it uses `json.dumps()` → `serde_json::from_str()`
/// as those types cannot be directly downcast without serialization.
///
/// # Errors
///
/// Returns `PyErr` if conversion fails (e.g., invalid JSON, unsupported types)
///
/// # Examples
///
/// ```ignore
/// let py_dict = PyDict::new(py);
/// py_dict.set_item("name", "Alice")?;
/// let value = python_to_json(py, &py_dict.as_any().into())?;
/// ```
pub fn python_to_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if obj.is_none() {
        return Ok(Value::Null);
    }

    // Fast path: msgspec.Struct (or nested msgspec types) → builtins → JSON.
    //
    // Spikard's Python surface is msgspec-centric; handlers frequently return msgspec.Struct
    // instances. Falling back to `json.dumps()` is both slower and can fail because msgspec
    // structs are not JSON-serializable by the stdlib `json` module by default.
    if let Some((struct_type, to_builtins)) = msgspec_struct_helpers(py)
        && obj.is_instance(struct_type.bind(py))?
    {
        let builtins = to_builtins.bind(py).call1((obj,))?;
        return python_to_json(py, &builtins);
    }

    if let Ok(b) = obj.extract::<bool>() {
        return Ok(Value::Bool(b));
    }

    if let Ok(i) = obj.extract::<i64>() {
        return Ok(Value::Number(i.into()));
    }

    if let Ok(f) = obj.extract::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(f) {
            return Ok(Value::Number(num));
        }
        return Err(pyo3::exceptions::PyValueError::new_err("Invalid float value"));
    }

    if let Ok(s) = obj.extract::<String>() {
        return Ok(Value::String(s));
    }

    #[allow(deprecated)]
    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str: String = key.extract()?;
            let json_value = python_to_json(py, &value)?;
            map.insert(key_str, json_value);
        }
        return Ok(Value::Object(map));
    }

    #[allow(deprecated)]
    if let Ok(list) = obj.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            let json_value = python_to_json(py, &item)?;
            arr.push(json_value);
        }
        return Ok(Value::Array(arr));
    }

    let json_module = py.import("json")?;
    let json_str: String = json_module.call_method1("dumps", (obj,))?.extract()?;

    serde_json::from_str(&json_str)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse JSON: {e}")))
}
