use pyo3::prelude::*;

mod xls2dat;

#[pymodule]
pub fn compass(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(xls2dat::convert_xls_json_to_dat, m)?)?;
    Ok(())
}
