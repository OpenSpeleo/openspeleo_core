use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyFloat, PyList, PyString},
};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};
use std::collections::HashMap;

use pyo3_stub_gen::derive::gen_stub_pyfunction;

// Python bindings with optional null field preservation

#[gen_stub_pyfunction(module = "openspeleo_core._lib.ariane")]
#[pyfunction]
pub fn xml_str_to_dict(xml_str: &str, keep_null: bool) -> PyResult<PyObject> {
    let value = parse_xml(xml_str, keep_null)
        .map_err(|e| PyValueError::new_err(format!("XML parsing error: {}", e)))?;
    Python::with_gil(|py| value_to_pyobject(&value, py))
}

// XML to Dict implementation with optional null field preservation
fn parse_xml(xml: &str, keep_null: bool) -> Result<Value, String> {
    // Create a new XML reader
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    // Initialize variables to keep track of the parsing state
    let mut stack: Vec<(String, Option<Value>, HashMap<String, Value>)> = Vec::new();
    let mut root: Option<Value> = None;
    let mut current_value: Option<Value> = None;
    let mut current_attrs: HashMap<String, Value> = HashMap::new();
    let mut buf = Vec::new();
    let mut root_name = String::new();

    loop {
        // Read the next event from the XML reader
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                // Handle the start of an element
                let name = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name).into_owned();

                // Set the root name if it's not already set
                if root_name.is_empty() {
                    root_name = name.clone();
                }

                // Handle attributes
                let attrs: HashMap<String, Value> = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .map(|a| {
                        let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let value = a.unescape_value().unwrap_or_default().to_string();
                        (format!("@{}", key), Value::String(value))
                    })
                    .collect();

                // Push the current state onto the stack
                stack.push((name, current_value, current_attrs));
                current_attrs = attrs;
                current_value = Some(Value::Object(Map::new()));
            }
            Ok(Event::Text(e)) => {
                // Handle text content
                let text = e.unescape().unwrap_or_default().to_string();
                if !text.trim().is_empty() {
                    current_value = Some(Value::String(text));
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

                // Create a new value from the object
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
                let name = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name).into_owned();

                // Set the root name if it's not already set
                if root_name.is_empty() {
                    root_name = name.clone();
                }

                // Handle attributes
                let attrs: HashMap<String, Value> = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .map(|a| {
                        let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let value = a.unescape_value().unwrap_or_default().to_string();
                        (format!("@{}", key), Value::String(value))
                    })
                    .collect();

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
