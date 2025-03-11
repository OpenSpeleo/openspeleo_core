use pyo3::prelude::*;

use pyo3::wrap_pymodule;
use pyo3_stub_gen::define_stub_info_gatherer;

pub mod ariane;
mod utils;

#[pymodule]
fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(ariane::ariane))?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
