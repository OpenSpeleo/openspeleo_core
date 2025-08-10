use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyList, PyListMethods};
use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[gen_stub_pyfunction(module = "openspeleo_core._rust_lib.mapping")]
#[pyfunction]
pub fn apply_key_mapping(
    py: Python,
    data: Bound<'_, PyAny>,
    mapping: Bound<'_, PyDict>,
) -> PyResult<PyObject> {
    apply_key_mapping_optimized(py, &data, &mapping)
}

#[inline]
fn apply_key_mapping_optimized(
    py: Python,
    data: &Bound<'_, PyAny>,
    mapping: &Bound<'_, PyDict>,
) -> PyResult<PyObject> {
    // Step 3: Fast type dispatch using raw type pointer comparison
    let data_type = unsafe { pyo3::ffi::Py_TYPE(data.as_ptr()) };

    if std::ptr::eq(
        data_type as *const _,
        &raw const pyo3::ffi::PyDict_Type as *const _,
    ) {
        let dict = data.downcast::<PyDict>()?;
        let result = PyDict::new(py);

        // Step 2: Use raw PyDict_Next iteration like Cython does
        unsafe {
            let mut pos: pyo3::ffi::Py_ssize_t = 0;
            let mut key_ptr: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
            let mut value_ptr: *mut pyo3::ffi::PyObject = std::ptr::null_mut();

            // Raw C API iteration - exactly like Cython
            while pyo3::ffi::PyDict_Next(dict.as_ptr(), &mut pos, &mut key_ptr, &mut value_ptr) != 0
            {
                let key: Bound<'_, PyAny> = Bound::from_borrowed_ptr(py, key_ptr);
                let value: Bound<'_, PyAny> = Bound::from_borrowed_ptr(py, value_ptr);

                // Direct dict lookup - mimic Cython's mapping.get(key, key)
                let mapped_key = match mapping.get_item(&key) {
                    Ok(Some(mapped)) => mapped,
                    _ => key.clone(),
                };

                // Step 3: Fast type check using raw pointers instead of is_instance_of
                let value_type = pyo3::ffi::Py_TYPE(value_ptr);
                let processed_value = if std::ptr::eq(
                    value_type as *const _,
                    &raw const pyo3::ffi::PyDict_Type as *const _,
                ) || std::ptr::eq(
                    value_type as *const _,
                    &raw const pyo3::ffi::PyList_Type as *const _,
                ) {
                    apply_key_mapping_optimized(py, &value, mapping)?
                } else {
                    value.unbind()
                };

                result.set_item(&mapped_key, &processed_value)?;
            }
        }

        Ok(result.unbind().into())
    } else if std::ptr::eq(
        data_type as *const _,
        &raw const pyo3::ffi::PyList_Type as *const _,
    ) {
        let list = data.downcast::<PyList>()?;
        let result = PyList::empty(py);

        for item in list {
            // Step 3: Fast type check for list items
            let item_type = unsafe { pyo3::ffi::Py_TYPE(item.as_ptr()) };
            let processed_item = if std::ptr::eq(
                item_type as *const _,
                &raw const pyo3::ffi::PyDict_Type as *const _,
            ) || std::ptr::eq(
                item_type as *const _,
                &raw const pyo3::ffi::PyList_Type as *const _,
            ) {
                apply_key_mapping_optimized(py, &item, mapping)?
            } else {
                item.unbind()
            };

            result.append(&processed_item)?;
        }

        Ok(result.unbind().into())
    } else {
        // Return primitive values as-is
        Ok(data.clone().unbind())
    }
}

#[pymodule]
pub fn mapping(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_key_mapping, m)?)?;
    Ok(())
}
