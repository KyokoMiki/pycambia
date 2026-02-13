mod py_classes;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use py_classes::{PyCambiaResponse, PyRipper};

/// Parse a CD ripping log file and return typed Python objects.
///
/// Accepts either a string path or a PathLike object (e.g., pathlib.Path).
#[pyfunction]
fn parse_log_file(py: Python<'_>, path: &Bound<'_, PyAny>) -> PyResult<PyCambiaResponse> {
    // Try to extract as PathBuf first (handles pathlib.Path and similar)
    let path_buf: std::path::PathBuf = if let Ok(p) = path.extract::<std::path::PathBuf>() {
        p
    } else if let Ok(s) = path.extract::<String>() {
        // Fallback to string extraction
        std::path::PathBuf::from(s)
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "path must be str or PathLike",
        ));
    };

    let response = py.detach(|| {
        let raw = std::fs::read(&path_buf).map_err(|e| {
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
fn get_supported_rippers() -> PyResult<Vec<PyRipper>> {
    #[allow(unused_mut)]
    let mut rippers = vec![PyRipper::EAC, PyRipper::XLD, PyRipper::Whipper];

    #[cfg(feature = "experimental_rippers")]
    rippers.push(PyRipper::CueRipper);

    Ok(rippers)
}

/// A Python module implemented in Rust.
#[pymodule(gil_used = false)]
fn _cambia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_log_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_log_content, m)?)?;
    m.add_function(wrap_pyfunction!(get_supported_rippers, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    py_classes::register_classes(m)?;
    Ok(())
}
