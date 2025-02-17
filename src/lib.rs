use pyo3::prelude::*;
use std::process::Command;

/// Parses an HDFC credit card statement using the CLI and returns the output.
#[pyfunction]
fn parse_cc_statement(file_path: &str, name: &str, password: &str) -> PyResult<String> {
    let output = Command::new("hdfc-cc-parser-rs") // Assuming the binary is named like this
        .arg(format!("--name={}", name))
        .arg(format!("--file={}", file_path))
        .arg(format!("--password={}", password))
        .output()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to run command: {}", e)))?;

    if !output.status.success() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            format!("Error: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Create the Python module
#[pymodule]
fn hdfc_cc_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_cc_statement, m)?)?;
    Ok(())
}

