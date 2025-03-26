# XML to Dict Converter

This Rust library provides functions to convert an XML string to a Python dictionary and back.

## Installation

To build the Python package, run:
```sh
maturin develop
```

## Usage

```python
import xml_to_dict

with open("demo.xml", "r") as xml_file:
    xml_str = xml_file.read()

dict_str = xml_to_dict.xml_str_to_dict(xml_str)
print("Dictionary representation:")
print(dict_str)

with open("demo.json", "r") as json_file:
    expected_dict_str = json_file.read()

assert dict_str == expected_dict_str, "Conversion to dict failed"

xml_str_back = xml_to_dict.dict_to_xml_str(dict_str)
print("XML representation:")
print(xml_str_back)

assert xml_str_back == xml_str, "Conversion back to XML failed"
```
