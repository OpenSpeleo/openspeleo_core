use ahash::AHashMap;
use pyo3::{exceptions::PyValueError, prelude::*};
use pythonize::pythonize;
use quick_xml::escape::resolve_predefined_entity;
use quick_xml::events::{BytesRef, BytesStart, Event};
use quick_xml::Reader;
use serde_json::{Map, Value};

use pyo3_stub_gen::derive::gen_stub_pyfunction;

/// Resolves an entity reference to its string representation.
/// Handles both predefined entities (lt, gt, amp, apos, quot) and character references (&#60; or &#x3C;).
fn resolve_entity_ref(entity: &BytesRef<'_>) -> Option<String> {
    // First, try to resolve as a character reference (&#60; or &#x3C;)
    if let Ok(Some(c)) = entity.resolve_char_ref() {
        return Some(c.to_string());
    }

    // Try to resolve as a predefined entity (lt, gt, amp, apos, quot)
    let entity_name = std::str::from_utf8(entity.as_ref()).ok()?;
    resolve_predefined_entity(entity_name).map(|s| s.to_string())
}

// Python bindings with optional null field preservation

#[gen_stub_pyfunction(module = "openspeleo_core._rust_lib.ariane")]
#[pyfunction]
pub fn xml_str_to_dict(xml_str: &str, keep_null: bool) -> PyResult<Py<PyAny>> {
    let value = parse_xml(xml_str, keep_null)
        .map_err(|e| PyValueError::new_err(format!("XML parsing error: {e}")))?;
    Python::attach(|py| Ok(pythonize(py, &value)?.into()))
}

fn collect_attrs(e: &BytesStart<'_>) -> AHashMap<String, Value> {
    let iter = e.attributes();
    let mut map = AHashMap::with_capacity(iter.size_hint().1.unwrap_or(0));

    for attr in iter.filter_map(Result::ok) {
        // Safety: According to XML spec and quick_xml guarantees, element and attribute names are valid UTF-8
        let key = unsafe { std::str::from_utf8_unchecked(attr.key.as_ref()) };

        let mut full_key = String::with_capacity(1 + key.len());
        full_key.push('@');
        full_key.push_str(key);

        let value = attr.unescape_value().unwrap_or_default().into_owned();

        map.insert(full_key, Value::String(value));
    }
    map
}

// XML to Dict implementation with optional null field preservation
fn parse_xml(xml: &str, keep_null: bool) -> Result<Value, String> {
    // Create a new XML reader with optimizations
    let mut reader = Reader::from_str(xml);
    // NOTE: trim_text must be false in quick-xml 0.38+ because text is now split across
    // multiple events (Text + GeneralRef), and trimming each event individually would
    // lose internal whitespace. We accumulate all fragments first, then trim only
    // the leading/trailing whitespace from the complete text when flushing the buffer.
    reader.config_mut().trim_text(false);
    reader.config_mut().check_end_names = false;
    reader.config_mut().expand_empty_elements = false;

    // Initialize variables to keep track of the parsing state
    let mut stack: Vec<(String, Option<Value>, AHashMap<String, Value>)> = Vec::with_capacity(32);
    let mut root: Option<Value> = None;
    let mut current_value: Option<Value> = None;
    let mut current_attrs: AHashMap<String, Value> = AHashMap::default();
    let mut buf = Vec::with_capacity(1024);
    let mut root_name = String::new();
    // Text accumulator for consecutive Text/GeneralRef events (needed for quick-xml 0.38+)
    let mut text_buffer = String::new();

    loop {
        // Read the next event from the XML reader
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                // Flush any accumulated text before processing new element
                if !text_buffer.is_empty() {
                    // Trim leading/trailing whitespace from the complete accumulated text
                    let trimmed = text_buffer.trim();
                    if !trimmed.is_empty() {
                        current_value = Some(Value::String(trimmed.to_owned()));
                    }
                    text_buffer.clear();
                }

                // Handle the start of an element

                // Safety: According to XML spec and quick_xml guarantees, element and attribute names are valid UTF-8
                let name = unsafe { std::str::from_utf8_unchecked(e.name().as_ref()) }.to_string();

                // Set the root name if it's not already set
                if root_name.is_empty() {
                    root_name = name.clone();
                }

                // Handle attributes efficiently with pre-allocation
                let attrs = collect_attrs(&e);

                // Push the current state onto the stack
                stack.push((name, current_value, current_attrs));
                current_attrs = attrs;
                current_value = Some(Value::Object(Map::new()));
            }
            Ok(Event::Text(e)) => {
                // Handle text content - accumulate in buffer (quick-xml 0.38+ splits text on entity refs)
                if let Ok(text) = e.decode() {
                    text_buffer.push_str(&text);
                }
            }
            Ok(Event::GeneralRef(e)) => {
                // Handle entity references like &lt; &gt; &amp; etc. (new in quick-xml 0.38+)
                if let Some(resolved) = resolve_entity_ref(&e) {
                    text_buffer.push_str(&resolved);
                }
            }
            Ok(Event::End(_)) => {
                // Flush any accumulated text before processing end element
                if !text_buffer.is_empty() {
                    // Trim leading/trailing whitespace from the complete accumulated text
                    let trimmed = text_buffer.trim();
                    if !trimmed.is_empty() {
                        current_value = Some(Value::String(trimmed.to_owned()));
                    }
                    text_buffer.clear();
                }

                // Handle the end of an element
                // let (name, parent_val, parent_attrs) = stack.pop().unwrap();
                let (name, parent_val, parent_attrs) = match stack.pop() {
                    Some(t) => t,
                    None => return Err("Unexpected end tag without matching start".to_string()),
                };

                let mut obj = match current_value.take() {
                    Some(Value::Object(m)) => m,
                    Some(v) => {
                        let mut m = Map::new();
                        m.insert("#text".to_string(), v);
                        m
                    }
                    None => Map::new(),
                };

                obj.extend(current_attrs);

                current_value = parent_val;
                current_attrs = parent_attrs;

                // Create a new value from the object - optimize for single text content
                let new_value = if obj.len() == 1 {
                    obj.remove("#text").unwrap_or(Value::Object(obj))
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
                // Flush any accumulated text before processing empty element
                if !text_buffer.is_empty() {
                    // Trim leading/trailing whitespace from the complete accumulated text
                    let trimmed = text_buffer.trim();
                    if !trimmed.is_empty() {
                        current_value = Some(Value::String(trimmed.to_owned()));
                    }
                    text_buffer.clear();
                }

                // Handle empty elements
                // Safety: According to XML spec and quick_xml guarantees, element and attribute names are valid UTF-8
                let name = unsafe { std::str::from_utf8_unchecked(e.name().as_ref()) }.to_string();

                // Set the root name if it's not already set
                if root_name.is_empty() {
                    root_name = name.clone();
                }

                // Handle attributes with pre-allocation
                let attrs = collect_attrs(&e);

                // Create a new value from the attributes
                let new_value = if keep_null {
                    Value::Null
                } else if attrs.is_empty() {
                    // Skip this empty element without attributes
                    buf.clear();
                    continue;
                } else {
                    Value::Object(attrs.into_iter().collect())
                };

                // Check if the new value is null and if we should keep null values
                if keep_null || new_value != Value::Null {
                    // Check if the new value is an empty object and if we should keep null values
                    if let Value::Object(ref obj) = new_value {
                        if obj.is_empty() && !keep_null {
                            buf.clear();
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
                                let existing_val = std::mem::replace(existing, Value::Null);
                                let arr = vec![existing_val, new_value];
                                parent.insert(name.clone(), Value::Array(arr));
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
