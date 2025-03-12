use pyo3::prelude::*;

use pyo3_stub_gen::derive::gen_stub_pyfunction;

use super::deserialize;

/// Reads the contents of the "Data.xml" file from a zip archive.
///
/// # Arguments
///
/// * `path`: The path to the zip archive.
///
/// # Returns
///
/// The contents of the "Data.xml" file as a string.
#[gen_stub_pyfunction(module = "openspeleo_core._lib.ariane")]
#[pyfunction]
pub fn load_ariane_tml_file_to_dict(path: &str) -> PyResult<PyObject> {
    let file = std::fs::File::open(path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open file: {}", e))
    })?;
    let reader = std::io::BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open zip archive: {}", e))
    })?;

    let mut xml_file = archive.by_name("Data.xml").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
            "Failed to find file in zip archive: {}",
            e
        ))
    })?;

    let mut xml_contents_vec = Vec::new();
    std::io::copy(&mut xml_file, &mut xml_contents_vec).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e))
    })?;

    // Re-Allocation to change type from Vec<u8> to String
    let xml_contents = String::from_utf8(xml_contents_vec).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyUnicodeError, _>(format!(
            "Failed to convert bytes to string: {}",
            e
        ))
    })?;

    // Convert XML to dict
    Ok(deserialize::xml_str_to_dict(xml_contents.as_str(), false)?)
}
