import json
import pprint

import xml_dict

with open("demo.xml", "r") as xml_file:
    xml_str = xml_file.read()

data = xml_dict.xml_str_to_dict(xml_str)
print("Dictionary representation:")
pprint.pprint(data)

with open("demo.json", "r") as json_file:
    expected_data = json.load(json_file)

assert data == expected_data, "Conversion to dict failed"

xml_str_back = xml_dict.dict_to_xml_str(data)
print("XML representation:")
print(xml_str_back)

assert xml_str_back == xml_str, "Conversion back to XML failed"
