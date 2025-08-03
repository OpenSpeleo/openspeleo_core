use ahash::AHashMap;
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyFloat, PyList, PyString},
};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};

use pyo3_stub_gen::derive::gen_stub_pyfunction;

// Python bindings with optional null field preservation

#[gen_stub_pyfunction(module = "openspeleo_core._rust_lib.ariane")]
#[pyfunction]
pub fn xml_str_to_dict(xml_str: &str, keep_null: bool) -> PyResult<PyObject> {
    let value = parse_xml(xml_str, keep_null)
        .map_err(|e| PyValueError::new_err(format!("XML parsing error: {}", e)))?;
    Python::with_gil(|py| value_to_pyobject(&value, py))
}

// XML to Dict implementation with optional null field preservation
fn parse_xml(xml: &str, keep_null: bool) -> Result<Value, String> {
    // Create a new XML reader with optimizations
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    reader.config_mut().check_end_names = false;
    reader.config_mut().expand_empty_elements = false;

    // Initialize variables to keep track of the parsing state
    let mut stack: Vec<(String, Option<Value>, AHashMap<String, Value>)> = Vec::with_capacity(32);
    let mut root: Option<Value> = None;
    let mut current_value: Option<Value> = None;
    let mut current_attrs: AHashMap<String, Value> = AHashMap::default();
    let mut buf = Vec::with_capacity(1024);
    let mut root_name = String::new();

    loop {
        // Read the next event from the XML reader
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                // Handle the start of an element
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();

                // Set the root name if it's not already set
                if root_name.is_empty() {
                    root_name = name.clone();
                }

                // Handle attributes efficiently with pre-allocation
                let attrs_iter = e.attributes();
                let size_hint = attrs_iter.size_hint();
                let mut attrs = AHashMap::with_capacity(size_hint.1.unwrap_or(size_hint.0));

                for attr in attrs_iter.filter_map(|a| a.ok()) {
                    let key = String::from_utf8_lossy(attr.key.as_ref());
                    let value = attr.unescape_value().unwrap_or_default();
                    attrs.insert(format!("@{}", key), Value::String(value.into_owned()));
                }

                // Push the current state onto the stack
                stack.push((name, current_value, current_attrs));
                current_attrs = attrs;
                current_value = Some(Value::Object(Map::new()));
            }
            Ok(Event::Text(e)) => {
                // Handle text content
                
                // Does not work - Creates a mismatch error with python implementation
                // let text = String::from_utf8_lossy(&e);
                // if !text.trim().is_empty() {
                //     current_value = Some(Value::String(text.into_owned()));
                // }

                // The following line does not work with `quick-xml` 0.38
                let text = e.unescape().unwrap_or_default().to_string();
                if !text.trim().is_empty() {
                    current_value = Some(Value::String(text.to_owned()));
                }
            }
            Ok(Event::End(_)) => {
                // Handle the end of an element
                let (name, parent_val, parent_attrs) = stack.pop().unwrap();
                let mut obj = match current_value.take() {
                    Some(Value::Object(m)) => m,
                    Some(v) => {
                        let mut m = Map::new();
                        m.insert("#text".to_string(), v);
                        m
                    }
                    None => Map::new(),
                };

                // Merge attributes
                for (k, v) in current_attrs.drain() {
                    obj.insert(k, v);
                }

                current_value = parent_val;
                current_attrs = parent_attrs;

                // Create a new value from the object - optimize for single text content
                let new_value = if obj.len() == 1 && obj.contains_key("#text") {
                    obj.remove("#text").unwrap()
                } else {
                    Value::Object(obj)
                };

                // Check if the new value is null and if we should keep null values
                if keep_null || new_value != Value::Null {
                    // Check if the new value is an empty object and if we should keep null values
                    if let Value::Object(ref obj) = new_value {
                        if obj.is_empty() && !keep_null {
                            continue;
                        }
                    }

                    // Add the new value to the parent object
                    if let Some(Value::Object(ref mut parent)) = current_value {
                        // Handle duplicate keys by converting to array
                        if let Some(existing) = parent.get_mut(&name) {
                            if let Value::Array(ref mut arr) = existing {
                                arr.push(new_value);
                            } else {
                                let existing_val = existing.take();
                                parent.insert(name, Value::Array(vec![existing_val, new_value]));
                            }
                        } else {
                            parent.insert(name, new_value);
                        }
                    } else {
                        root = Some(new_value);
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                // Handle empty elements
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();

                // Set the root name if it's not already set
                if root_name.is_empty() {
                    root_name = name.clone();
                }

                // Handle attributes with pre-allocation
                let attrs_iter = e.attributes();
                let size_hint = attrs_iter.size_hint();
                let mut attrs = AHashMap::with_capacity(size_hint.1.unwrap_or(size_hint.0));

                for attr in attrs_iter.filter_map(|a| a.ok()) {
                    let key = String::from_utf8_lossy(attr.key.as_ref());
                    let value = attr.unescape_value().unwrap_or_default();
                    attrs.insert(format!("@{}", key), Value::String(value.into_owned()));
                }

                // Create a new value from the attributes
                let new_value = if keep_null {
                    Value::Null
                } else {
                    if attrs.is_empty() {
                        continue;
                    } else {
                        let mut obj = Map::new();
                        for (k, v) in attrs {
                            obj.insert(k, v);
                        }
                        if obj.is_empty() {
                            continue;
                        }
                        Value::Object(obj)
                    }
                };

                // Check if the new value is null and if we should keep null values
                if keep_null || new_value != Value::Null {
                    // Check if the new value is an empty object and if we should keep null values
                    if let Value::Object(ref obj) = new_value {
                        if obj.is_empty() && !keep_null {
                            continue;
                        }
                    }

                    // Add the new value to the parent object
                    if let Some(Value::Object(ref mut parent)) = current_value {
                        // Handle duplicate keys by converting to array
                        if let Some(existing) = parent.get_mut(&name) {
                            if let Value::Array(ref mut arr) = existing {
                                arr.push(new_value);
                            } else {
                                let existing_val = existing.take();
                                parent.insert(name, Value::Array(vec![existing_val, new_value]));
                            }
                        } else {
                            parent.insert(name, new_value);
                        }
                    } else {
                        root = Some(new_value);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                // Handle errors
                return Err(format!(
                    "Error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                ));
            }
            _ => (),
        }
        buf.clear();
    }

    // Create the final root object
    root.map(|r| {
        let mut root_obj = Map::new();
        root_obj.insert(root_name, r);
        Value::Object(root_obj)
    })
    .ok_or_else(|| "Empty XML document".to_string())
}

// Updated helper functions for Python/Rust type conversion

// Function to handle conversion of serde_json::Value
fn value_to_pyobject(value: &Value, py: Python<'_>) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.into_pyobject(py).unwrap().to_owned().into()),
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(i.into_pyobject(py).unwrap().to_owned().into())
            } else if let Some(f) = num.as_f64() {
                Ok(PyFloat::new(py, f).into())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err("Invalid number"))
            }
        }
        Value::String(s) => Ok(PyString::new(py, s).into()),
        Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(value_to_pyobject(item, py)?)?;
            }
            Ok(list.into())
        }
        Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, value_to_pyobject(v, py)?)?;
            }
            Ok(dict.into())
        }
    }
}
