import json
import pprint
from pathlib import Path

import xml_dict
import xmltodict
from deepdiff import DeepDiff

with open("demo.xml", "r") as xml_file:
    xml_str = xml_file.read()

data = xml_dict.xml_str_to_dict(xml_str)
with Path("demo.out.json").open("w") as json_file:
    json.dump(data, json_file, indent=4, sort_keys=True)

# with open("demo.json", "r") as json_file:
#     expected_data = json.load(json_file)
expected_data = xmltodict.parse(xml_str)
with Path("demo.json").open("w") as json_file:
    json.dump(expected_data, json_file, indent=4, sort_keys=True)

diff = DeepDiff(data, expected_data, ignore_order=True)
assert diff == {}, f"Identity Check failed: {diff}"

# xml_str_back = xml_dict.dict_to_xml_str(data)
# print("XML representation:")
# print(xml_str_back)

# assert xml_str_back == xml_str, "Conversion back to XML failed"
