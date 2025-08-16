#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyModule};
#[cfg(feature = "python")]
use pyo3::Bound;
#[cfg(feature = "python")]
use pythonize::pythonize;

pub mod util;

// Simple Args struct for library mode when not using Python
#[cfg(not(feature = "python"))]
pub struct Args {
    pub save_logs: Option<std::path::PathBuf>,
}

#[cfg(feature = "python")]
fn cambia_response_to_py(py: Python, response: &cambia_core::response::CambiaResponse) -> PyResult<PyObject> {
    pythonize(py, response)
        .map(|obj| obj.into())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Conversion error: {}", e)))
}

/// Parse a CD ripping log file and return the parsed data as a Python dictionary
#[cfg(feature = "python")]
#[pyfunction]
fn parse_log_file(py: Python, path: String) -> PyResult<PyObject> {
    let result = util::parse_file_for_python(&path);
    
    match result {
        Ok(data) => {
            let dict = PyDict::new(py);
            
            dict.set_item("success", true)?;
            dict.set_item("data", cambia_response_to_py(py, &data)?)?;
            
            Ok(dict.into())
        }
        Err(e) => {
            let dict = PyDict::new(py);
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
            let dict = PyDict::new(py);
            dict.set_item("success", true)?;
            dict.set_item("data", cambia_response_to_py(py, &data)?)?;
            Ok(dict.into())
        }
        Err(e) => {
            let dict = PyDict::new(py);
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