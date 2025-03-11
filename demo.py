import json
import zipfile
from pathlib import Path
from pprint import pprint

import xmltodict
from deepdiff import DeepDiff

from openspeleo_core import mapping

# filepath = Path("tests/artifacts/test_simple.mini.tml")
# with zipfile.ZipFile(filepath, "r") as zip_file:
#     xml_str = zip_file.open("Data.xml", mode="r").read().decode("utf-8")

# with Path("demo.normal.json").open("w") as json_file:
#     json.dump(ariane_core.xml_str_to_dict(xml_str), json_file, indent=4, sort_keys=True)

# with Path("demo.cleaned.json").open("w") as json_file:
#     json.dump(
#         ariane_core.xml_str_to_dict(xml_str, keep_null=False),
#         json_file,
#         indent=4,
#         sort_keys=True,
#     )

data = {"Azimut": "0.0", "Depth": "10.0", "Explorer": "Ariane"}
mapping_dict = {"Azimut": "Bearing", "Explorer": "Diver"}
expected_output = {"Bearing": "0.0", "Depth": "10.0", "Diver": "Ariane"}
print(f"Result:   {mapping.apply_key_mapping(data, mapping_dict)}")
print(f"Expected: {expected_output}")
