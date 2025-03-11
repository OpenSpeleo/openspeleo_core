use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use std::collections::BTreeMap;

fn apply_key_mapping_in_place(
    py: Python,
    data: Py<PyAny>,
    mapping: &BTreeMap<String, String>,
) -> PyResult<PyObject> {
    if let Ok(dict) = data.downcast_bound::<PyDict>(py) {
        let new_dict = PyDict::new(py);
        for (key, val) in dict {
            let key_str: String = key.extract()?;
            let new_key = mapping.get(&key_str).unwrap_or(&key_str).to_string();
            let new_val = apply_key_mapping_in_place(py, val.into(), mapping)?;
            new_dict.set_item(new_key, new_val)?;
        }
        Ok(new_dict.into())
    } else if let Ok(list) = data.downcast_bound::<PyList>(py) {
        let new_list = PyList::empty(py);
        for val in list {
            let new_val = apply_key_mapping_in_place(py, val.into(), mapping)?;
            new_list.append(new_val)?;
        }
        Ok(new_list.into())
    } else {
        Ok(data.into())
    }
}

#[gen_stub_pyfunction(module = "openspeelo_core._lib.mapping")]
#[pyfunction]
fn apply_key_mapping(
    py: Python,
    data: Py<PyAny>,
    mapping: BTreeMap<String, String>,
) -> PyResult<PyObject> {
    apply_key_mapping_in_place(py, data, &mapping)
}

#[pymodule]
pub fn mapping(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_key_mapping, m)?)?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pyo3::types::IntoPyDict;

//     #[test]
//     fn test_apply_key_mapping() {
//         Python::with_gil(|py| {
//             let data =
//                 [("Azimut", "0.0"), ("Depth", "10.0"), ("Explorer", "Ariane")].into_py_dict(py);
//             let mapping = [("Azimut", "Bearing"), ("Explorer", "Diver")]
//                 .iter()
//                 .cloned()
//                 .collect();

//             let result = apply_key_mapping_py(py, data, mapping).unwrap();
//             let expected =
//                 [("Bearing", "0.0"), ("Depth", "10.0"), ("Diver", "Ariane")].into_py_dict(py);

//             assert!(result.cast_as::<PyDict>().unwrap().eq(expected));
//         });
//     }
// }
