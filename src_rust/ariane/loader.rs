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
#[gen_stub_pyfunction(module = "openspeleo_core._rust_lib.ariane")]
#[pyfunction]
pub fn load_ariane_tml_file_to_dict(path: &str) -> PyResult<PyObject> {
    let file = std::fs::File::open(path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open file: {e}"))
    })?;
    // Use larger buffer for better I/O performance (64KiB instead of default 8KiB)
    let reader = std::io::BufReader::with_capacity(65_536, file);

    let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open zip archive: {e}"))
    })?;

    let mut xml_file = archive.by_name("Data.xml").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
            "Failed to find file in zip archive: {e}"
        ))
    })?;

    // Pre-allocate based on file size if available
    let file_size = xml_file.size() as usize;
    let mut xml_contents = String::with_capacity(file_size);

    // Read directly into string to avoid Vec<u8> -> String conversion
    std::io::Read::read_to_string(&mut xml_file, &mut xml_contents).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {e}"))
    })?;

    // Convert XML to dict
    deserialize::xml_str_to_dict(&xml_contents, false)
}
