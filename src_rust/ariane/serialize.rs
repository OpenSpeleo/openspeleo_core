use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList},
};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde_json::{Map, Value};
use std::io::Cursor;

use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[gen_stub_pyfunction(module = "openspeleo_core._rust_lib.ariane")]
#[pyfunction]
pub fn dict_to_xml_str(data: &Bound<'_, PyDict>, root_name: &str) -> PyResult<String> {
    let value = pyobject_to_value(data)?;
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer
        .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0",
            Some("utf-8"),
            None,
        )))
        .map_err(|e| PyValueError::new_err(format!("XML writing error: {e}")))?;

    value_to_xml(&value, root_name, &mut writer)
        .map_err(|e| PyValueError::new_err(format!("XML generation error: {e}")))?;

    let xml_string = String::from_utf8(writer.into_inner().into_inner())
        .map_err(|e| PyValueError::new_err(format!("UTF-8 conversion error: {e}")))?;
    Ok(xml_string)
}

fn pyobject_to_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if let Ok(s) = obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(Value::Number(serde_json::Number::from(i)))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(Value::Number(serde_json::Number::from_f64(f).unwrap()))
    } else if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let arr: Vec<_> = list
            .iter()
            .map(|item| pyobject_to_value(&item))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Value::Array(arr))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = Map::new();
        for (k, v) in dict {
            let key = k.extract::<String>().unwrap();
            let value = pyobject_to_value(&v).unwrap();
            map.insert(key, value);
        }
        Ok(Value::Object(map))
    } else {
        Err(PyValueError::new_err("Unsupported Python type"))
    }
}

fn value_to_xml(
    value: &Value,
    parent_name: &str,
    writer: &mut Writer<Cursor<Vec<u8>>>,
) -> Result<(), String> {
    match value {
        Value::Object(obj) => {
            let elem = BytesStart::new(parent_name);
            writer
                .write_event(Event::Start(elem))
                .map_err(|e| e.to_string())?;
            for (k, v) in obj {
                value_to_xml(v, k, writer)?;
            }
            writer
                .write_event(Event::End(BytesEnd::new(parent_name)))
                .map_err(|e| e.to_string())?;
        }
        Value::Array(arr) => {
            for item in arr {
                value_to_xml(item, parent_name, writer)?;
            }
        }
        Value::String(s) => {
            writer
                .write_event(Event::Start(BytesStart::new(parent_name)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(s)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new(parent_name)))
                .map_err(|e| e.to_string())?;
        }
        Value::Number(n) => {
            writer
                .write_event(Event::Start(BytesStart::new(parent_name)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&n.to_string())))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new(parent_name)))
                .map_err(|e| e.to_string())?;
        }
        Value::Bool(b) => {
            writer
                .write_event(Event::Start(BytesStart::new(parent_name)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(if *b {
                    "true"
                } else {
                    "false"
                })))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new(parent_name)))
                .map_err(|e| e.to_string())?;
        }
        Value::Null => {
            writer
                .write_event(Event::Empty(BytesStart::new(parent_name)))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
