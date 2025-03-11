use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use quick_xml::events::Event;

use pyo3_stub_gen::derive::gen_stub_pyfunction;

use super::utils;

// Python bindings with optional null field preservation

#[gen_stub_pyfunction(module = "openspeleo_core._lib.ariane")]
#[pyfunction]
pub fn xml_str_to_dict(xml_str: &str, keep_null: bool) -> PyResult<PyObject> {
    let value = utils::parse_xml(xml_str, keep_null)
        .map_err(|e| PyValueError::new_err(format!("XML parsing error: {}", e)))?;
    Python::with_gil(|py| utils::value_to_pyobject(&value, py))
}

#[gen_stub_pyfunction(module = "openspeleo_core._lib.ariane")]
#[pyfunction]
pub fn dict_to_xml_str(data: &Bound<'_, PyDict>, root_name: &str) -> PyResult<String> {
    let value = utils::pyobject_to_value(data)?;
    let mut writer = quick_xml::Writer::new(Vec::new());
    writer
        .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0",
            Some("utf-8"),
            None,
        )))
        .map_err(|e| PyValueError::new_err(format!("XML writing error: {}", e)))?;

    utils::value_to_xml(&value, root_name, &mut writer)
        .map_err(|e| PyValueError::new_err(format!("XML generation error: {}", e)))?;

    String::from_utf8(writer.into_inner())
        .map_err(|e| PyValueError::new_err(format!("UTF-8 conversion error: {}", e)))
}

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

    // Convert str to dict
    let data = utils::parse_xml(xml_contents.as_str(), true)
        .map_err(|e| PyValueError::new_err(format!("XML parsing error: {}", e)))?;

    Python::with_gil(|py| utils::value_to_pyobject(&data, py))
}

#[pymodule]
#[pyo3(name = "ariane")]
pub fn ariane(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(xml_str_to_dict, m)?)?;
    m.add_function(wrap_pyfunction!(dict_to_xml_str, m)?)?;
    m.add_function(wrap_pyfunction!(load_ariane_tml_file_to_dict, m)?)?;
    Ok(())
}
