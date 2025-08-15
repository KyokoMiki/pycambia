#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyModule};
#[cfg(feature = "python")]
use pyo3::Bound;

pub mod util;

// Simple Args struct for library mode when not using Python
#[cfg(not(feature = "python"))]
pub struct Args {
    pub save_logs: Option<std::path::PathBuf>,
}

#[cfg(feature = "python")]
fn json_value_to_py(py: Python, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.to_object(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(n.to_string().to_object(py))
            }
        }
        serde_json::Value::String(s) => Ok(s.to_object(py)),
        serde_json::Value::Array(arr) => {
            let py_list = pyo3::types::PyList::empty_bound(py);
            for item in arr {
                py_list.append(json_value_to_py(py, item)?)?;
            }
            Ok(py_list.into())
        }
        serde_json::Value::Object(obj) => {
            let py_dict = PyDict::new_bound(py);
            for (key, value) in obj {
                py_dict.set_item(key, json_value_to_py(py, value)?)?;
            }
            Ok(py_dict.into())
        }
    }
}

/// Parse a CD ripping log file and return the parsed data as a Python dictionary
#[cfg(feature = "python")]
#[pyfunction]
fn parse_log_file(py: Python, path: String) -> PyResult<PyObject> {
    let result = util::parse_file_for_python(&path);
    
    match result {
        Ok(data) => {
            let dict = PyDict::new_bound(py);
            
            dict.set_item("success", true)?;
            dict.set_item("file_path", path)?;
            dict.set_item("data", json_value_to_py(py, &data)?)?;
            
            Ok(dict.into())
        }
        Err(e) => {
            let dict = PyDict::new_bound(py);
            dict.set_item("success", false)?;
            dict.set_item("error", format!("{}", e))?;
            Ok(dict.into())
        }
    }
}

/// Parse log content from a string and return the parsed data
#[cfg(feature = "python")]
#[pyfunction]
fn parse_log_content(py: Python, content: String) -> PyResult<PyObject> {
    let result = util::parse_content_for_python(&content);
    
    match result {
        Ok(data) => {
            let dict = PyDict::new_bound(py);
            dict.set_item("success", true)?;
            dict.set_item("data", json_value_to_py(py, &data)?)?;
            Ok(dict.into())
        }
        Err(e) => {
            let dict = PyDict::new_bound(py);
            dict.set_item("success", false)?;
            dict.set_item("error", format!("{}", e))?;
            Ok(dict.into())
        }
    }
}

/// Get supported log file formats
#[cfg(feature = "python")]
#[pyfunction]
fn get_supported_rippers() -> PyResult<Vec<String>> {
    Ok(cambia_core::handler::get_supported_rippers())
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
fn _cambia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_log_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_log_content, m)?)?;
    m.add_function(wrap_pyfunction!(get_supported_rippers, m)?)?;
    Ok(())
}