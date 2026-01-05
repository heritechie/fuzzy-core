use pyo3::prelude::*;

/// Compute fuzzy similarity score between two strings.
///
/// This is a thin Python wrapper over the Rust public API.
#[pyfunction]
fn similarity(a: &str, b: &str) -> PyResult<f64> {
    Ok(crate::similarity(a, b))
}

/// Python module definition
#[pymodule]
fn fuzzy_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(similarity, m)?)?;
    Ok(())
}
