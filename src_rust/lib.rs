use pyo3::prelude::*;

use pyo3::wrap_pymodule;
use pyo3_stub_gen::define_stub_info_gatherer;

mod ariane;
mod mapping;

#[pymodule]
fn _rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(ariane::ariane))?;
    m.add_wrapped(wrap_pymodule!(mapping::mapping))?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
