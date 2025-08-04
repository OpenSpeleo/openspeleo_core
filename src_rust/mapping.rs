use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};

use pyo3::ffi;
use std::ptr;

#[pyfunction]
fn apply_key_mapping(py: Python<'_>, data: &PyAny, mapping: &PyDict) -> PyResult<PyObject> {
    if let Ok(dict) = data.downcast::<PyDict>() {
        let out = PyDict::new(py);
        for (key, value) in dict.iter() {
            let new_key = mapping.get_item(key).unwrap_or(key);
            let new_value = apply_key_mapping(py, value, mapping)?;
            out.set_item(new_key, new_value)?;
        }
        Ok(out.into())
    } else if let Ok(list) = data.downcast::<PyList>() {
        let out = PyList::empty(py);
        for item in list.iter() {
            let new_item = apply_key_mapping(py, item, mapping)?;
            out.append(new_item)?;
        }
        Ok(out.into())
    } else {
        Ok(data.into())
    }
}

#[pyfunction]
unsafe fn apply_key_mapping_raw(
    py: Python<'_>,
    data: *mut ffi::PyObject,
    mapping: *mut ffi::PyObject,
) -> PyResult<PyObject> {
    if ffi::PyDict_Check(data) != 0 {
        let out = ffi::PyDict_New();
        let mut pos: ffi::Py_ssize_t = 0;
        let mut key: *mut ffi::PyObject = ptr::null_mut();
        let mut value: *mut ffi::PyObject = ptr::null_mut();

        while ffi::PyDict_Next(data, &mut pos, &mut key, &mut value) != 0 {
            let mapped_key = {
                let maybe = ffi::PyDict_GetItem(mapping, key);
                if maybe.is_null() {
                    key
                } else {
                    maybe
                }
            };

            let subvalue = apply_key_mapping_raw(py, value, mapping)?;
            ffi::PyDict_SetItem(out, mapped_key, subvalue.as_ptr());
        }

        Ok(PyObject::from_owned_ptr(py, out))
    } else if ffi::PyList_Check(data) != 0 {
        let len = ffi::PyList_GET_SIZE(data);
        let out = ffi::PyList_New(len);
        for i in 0..len {
            let item = ffi::PyList_GET_ITEM(data, i);
            let subvalue = apply_key_mapping_raw(py, item, mapping)?;
            ffi::Py_INCREF(subvalue.as_ptr());
            ffi::PyList_SET_ITEM(out, i, subvalue.as_ptr()); // Steals ref
        }
        Ok(PyObject::from_owned_ptr(py, out))
    } else {
        ffi::Py_INCREF(data);
        Ok(PyObject::from_owned_ptr(py, data))
    }
}

#[pymodule]
pub fn mapping(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_key_mapping, m)?)?;
    Ok(())
}
