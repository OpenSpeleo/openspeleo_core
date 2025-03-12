use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList},
};
use quick_xml::events::Event;
use serde_json::{Map, Value};

use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[gen_stub_pyfunction(module = "openspeleo_core._lib.ariane")]
#[pyfunction]
pub fn dict_to_xml_str(data: &Bound<'_, PyDict>, root_name: &str) -> PyResult<String> {
    let value = pyobject_to_value(data)?;
    let mut writer = quick_xml::Writer::new(Vec::new());
    writer
        .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0",
            Some("utf-8"),
            None,
        )))
        .map_err(|e| PyValueError::new_err(format!("XML writing error: {}", e)))?;

    value_to_xml(&value, root_name, &mut writer)
        .map_err(|e| PyValueError::new_err(format!("XML generation error: {}", e)))?;

    String::from_utf8(writer.into_inner())
        .map_err(|e| PyValueError::new_err(format!("UTF-8 conversion error: {}", e)))
}

fn pyobject_to_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if let Ok(s) = obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(n) = obj.extract::<f64>() {
        Ok(Value::from(n))
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(list) = obj.downcast::<PyList>() {
        // let mut arr = Vec::new();
        // for item in list.iter() {
        //     arr.push(pyobject_to_value(&item)?);
        // }
        let arr: Vec<_> = list
            .iter()
            .map(|item| pyobject_to_value(&item))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Value::Array(arr))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        // let mut map = Map::new();
        // for (k, v) in dict.iter() {
        //     let key: String = k.extract()?;
        //     map.insert(key, pyobject_to_value(&v)?);
        // }
        let map: Map<String, Value> = dict
            .iter()
            .filter_map(|(k, v)| {
                let key = k.extract().ok();
                let value = pyobject_to_value(&v).ok();
                key.zip(value).map(|(k, v)| (k, v))
            })
            .collect();
        Ok(Value::Object(map))
    } else {
        Err(PyValueError::new_err("Unsupported Python type").into())
    }
}

// Dict to XML implementation with root node preservation

fn value_to_xml(
    value: &Value,
    parent_name: &str,
    writer: &mut quick_xml::Writer<Vec<u8>>,
) -> Result<(), String> {
    let mut attributes = Vec::new();
    let mut children = Map::new();
    let mut text = None;

    if let Value::Object(obj) = value {
        for (k, v) in obj {
            if k.starts_with('@') {
                let attr_name = k.trim_start_matches('@');
                attributes.push((
                    attr_name.to_string(),
                    v.as_str().unwrap_or_default().to_string(),
                ));
            } else if k == "#text" {
                text = Some(v.as_str().unwrap_or_default().to_string());
            } else {
                children.insert(k.clone(), v.clone());
            }
        }
    }

    let mut elem = quick_xml::events::BytesStart::new(parent_name);
    for (name, value) in attributes {
        elem.push_attribute((name.as_str(), value.as_str()));
    }

    if children.is_empty() && text.is_none() {
        writer
            .write_event(Event::Empty(elem))
            .map_err(|e| e.to_string())?;
    } else {
        writer
            .write_event(Event::Start(elem))
            .map_err(|e| e.to_string())?;

        if let Some(text_content) = text {
            writer
                .write_event(Event::Text(quick_xml::events::BytesText::new(
                    &text_content,
                )))
                .map_err(|e| e.to_string())?;
        }

        for (name, value) in children {
            match value {
                Value::Array(arr) => {
                    for item in arr {
                        value_to_xml(&item, &name, writer)?;
                    }
                }
                _ => value_to_xml(&value, &name, writer)?,
            }
        }

        writer
            .write_event(Event::End(quick_xml::events::BytesEnd::new(parent_name)))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
