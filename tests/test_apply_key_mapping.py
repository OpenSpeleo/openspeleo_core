import unittest

import pytest
from openspeleo_core.mapping import apply_key_mapping


class TestApplyKeyMapping(unittest.TestCase):
    def test_dict_with_key_mapping(self):
        data = {"Azimuth": "0.0", "Depth": "10.0", "Explorer": "Ariane"}
        mapping = {"Azimuth": "Bearing", "Explorer": "Diver"}
        expected_output = {"Bearing": "0.0", "Depth": "10.0", "Diver": "Ariane"}
        assert apply_key_mapping(data, mapping) == expected_output

    def test_nested_dict_with_key_mapping(self):
        data = {
            "Shape": [
                {
                    "RadiusCollection": {
                        "RadiusVector": [
                            {"angle": "0.0", "length": "0.0"},
                            {"angle": "90.0", "length": "0.0"},
                        ]
                    },
                    "profileAzimuth": "0.0",
                }
            ],
            "Azimuth": "180.0",
        }
        mapping = {
            "Azimuth": "Bearing",
            "profileAzimuth": "profileBearing",
            "angle": "direction",
        }
        expected_output = {
            "Shape": [
                {
                    "RadiusCollection": {
                        "RadiusVector": [
                            {"direction": "0.0", "length": "0.0"},
                            {"direction": "90.0", "length": "0.0"},
                        ]
                    },
                    "profileBearing": "0.0",
                }
            ],
            "Bearing": "180.0",
        }
        assert apply_key_mapping(data, mapping) == expected_output

    def test_list_with_nested_dict_and_key_mapping(self):
        data = [
            {"Azimuth": "0.0", "Depth": "10.0"},
            {"Azimuth": "90.0", "Depth": "20.0"},
        ]
        mapping = {"Azimuth": "Bearing"}
        expected_output = [
            {"Bearing": "0.0", "Depth": "10.0"},
            {"Bearing": "90.0", "Depth": "20.0"},
        ]
        assert apply_key_mapping(data, mapping) == expected_output

    def test_no_key_mapping(self):
        data = {"Azimuth": "0.0", "Depth": "10.0"}
        mapping = {}
        assert apply_key_mapping(data, mapping) == data

    def test_no_match_key_mapping(self):
        data = {"Azimuth": "0.0", "Depth": "10.0"}
        mapping = {"NonexistentKey": "NewKey"}
        assert apply_key_mapping(data, mapping) == data

    def test_type_error_on_invalid_input(self):
        invalid = "Invalid data type"
        valid = {}

        with pytest.raises(
            TypeError, match="Unexpected type received for `data`: <class 'str'>"
        ):
            apply_key_mapping(invalid, valid)

        with pytest.raises(
            TypeError, match="Unexpected type received for `mapping`: <class 'str'>"
        ):
            apply_key_mapping(valid, invalid)

    def test_key_replacement_with_list(self):
        data = {"Azimuth": "0.0", "Depth": [1, 2, 3]}
        mapping = {"NonexistentKey": "NewKey"}
        assert apply_key_mapping(data, mapping) == data


if __name__ == "__main__":
    unittest.main()
