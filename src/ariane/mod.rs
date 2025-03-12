use pyo3::prelude::*;

mod deserialize;
mod loader;
mod serialize;

#[pymodule]
pub fn ariane(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(deserialize::xml_str_to_dict, m)?)?;
    m.add_function(wrap_pyfunction!(serialize::dict_to_xml_str, m)?)?;
    m.add_function(wrap_pyfunction!(loader::load_ariane_tml_file_to_dict, m)?)?;
    Ok(())
}
