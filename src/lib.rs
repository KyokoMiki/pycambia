mod py_classes;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use py_classes::PyCambiaResponse;

/// Parse a CD ripping log file and return typed Python objects.
#[pyfunction]
fn parse_log_file(py: Python<'_>, path: String) -> PyResult<PyCambiaResponse> {
    let response = py.detach(|| {
        let raw = std::fs::read(&path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("Could not read file: {}", e))
        })?;
        cambia_core::handler::parse_log_bytes(Vec::new(), &raw).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Could not parse log: {:?}", e))
        })
    })?;
    Ok(PyCambiaResponse::from_response(&response))
}

/// Parse log content from a string or bytes.
///
/// Accepts either `str` (converted to UTF-8 bytes) or `bytes` (used as-is
/// with automatic encoding detection by cambia-core).
#[pyfunction]
fn parse_log_content(content: &Bound<'_, PyAny>) -> PyResult<PyCambiaResponse> {
    let raw: Vec<u8> = if let Ok(s) = content.extract::<String>() {
        s.into_bytes()
    } else if let Ok(b) = content.extract::<Vec<u8>>() {
        b
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "content must be str or bytes",
        ));
    };

    let response = content.py().detach(|| {
        cambia_core::handler::parse_log_bytes(Vec::new(), &raw).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Could not parse log: {:?}", e))
        })
    })?;
    Ok(PyCambiaResponse::from_response(&response))
}

/// Get supported log file formats.
#[pyfunction]
fn get_supported_rippers() -> PyResult<Vec<String>> {
    #[allow(unused_mut)]
    let mut rippers = vec!["EAC".to_string(), "XLD".to_string(), "whipper".to_string()];

    #[cfg(feature = "experimental_rippers")]
    rippers.push("CUERipper".to_string());

    Ok(rippers)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _cambia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_log_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_log_content, m)?)?;
    m.add_function(wrap_pyfunction!(get_supported_rippers, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    py_classes::register_classes(m)?;
    Ok(())
}
