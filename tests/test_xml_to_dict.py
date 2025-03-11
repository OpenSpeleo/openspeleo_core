import json
import unittest
import zipfile
from pathlib import Path

import pytest
import xmltodict
from deepdiff import DeepDiff
from openspeleo_core import ariane_core
from parameterized import parameterized

DEBUG = False


def remove_none_values(d):
    """
    Recursively remove None values from a dictionary.

    Args:
        d (dict): The dictionary to clean.

    Returns:
        dict: The cleaned dictionary with no None values.
    """
    if isinstance(d, dict):
        for k, v in list(d.items()):
            if isinstance(v, (dict, tuple, list)):
                remove_none_values(v)
            if v is None:
                del d[k]

    elif isinstance(d, (list, tuple)):
        values = []
        for i in d:
            if i is None:
                continue
            if isinstance(i, (dict, tuple, list)):
                values.append(remove_none_values(i))
            else:
                values.append(i)

    return d


class TestCaseConversion(unittest.TestCase):
    def test_xml_str_to_dict(self):
        with Path("tests/artifacts/demo.xml").open("r") as xml_file:
            xml_str = xml_file.read()

        produced_data = ariane_core.xml_str_to_dict(xml_str, keep_null=True)
        if DEBUG:
            with Path("demo.produced.json").open("w") as json_file:
                json.dump(produced_data, json_file, indent=4, sort_keys=True)

        expected_data = xmltodict.parse(xml_str)
        with Path("demo.expected.json").open("w") as json_file:
            json.dump(expected_data, json_file, indent=4, sort_keys=True)

        diff = DeepDiff(produced_data, expected_data, ignore_order=True)
        assert diff == {}, f"Identity Check failed: {diff}"

    def test_xml_str_to_dict_no_null(self):
        with Path("tests/artifacts/demo.xml").open("r") as xml_file:
            xml_str = xml_file.read()

        produced_data = ariane_core.xml_str_to_dict(xml_str, keep_null=False)
        if DEBUG:
            with Path("demo.produced.json").open("w") as json_file:
                json.dump(produced_data, json_file, indent=4, sort_keys=True)

        expected_data = xmltodict.parse(xml_str)
        expected_data = remove_none_values(expected_data)
        with Path("demo.expected.json").open("w") as json_file:
            json.dump(expected_data, json_file, indent=4, sort_keys=True)

        diff = DeepDiff(produced_data, expected_data, ignore_order=True)
        assert diff == {}, f"Identity Check failed: {diff}"

    @parameterized.expand(["does_not_exists.xml", Path("does_not_exists.xml")])
    def test_load_ariane_tml_file_to_dict_no_file(self, filepath):
        with pytest.raises(FileNotFoundError):
            _ = ariane_core.load_ariane_tml_file_to_dict(filepath)

    @parameterized.expand(
        [
            ("tests/artifacts/hand_survey.tml",),
            ("tests/artifacts/test_simple.mini.tml",),
            ("tests/artifacts/test_simple.tml",),
            # ("tests/artifacts/test_simple.tmlu",),
            ("tests/artifacts/test_with_walls.tml",),
            ("tests/artifacts/test_large.tml",),
        ]
    )
    def test_load_ariane_tml_file_to_dict(self, filepath):
        # OpenSpeleo-Core
        produced_data = ariane_core.load_ariane_tml_file_to_dict(Path(filepath))

        # Naive Python Implementation
        with zipfile.ZipFile(filepath, "r") as zip_file:
            xml_str = zip_file.open("Data.xml", mode="r").read().decode("utf-8")
        expected_data = xmltodict.parse(xml_str)
        expected_data = remove_none_values(expected_data)

        diff = DeepDiff(produced_data, expected_data, ignore_order=True)
        assert diff == {}, f"Round trip transformation failed: {diff}"
