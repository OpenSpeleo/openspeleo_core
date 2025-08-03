from typing import Any


def remove_none_values(input_data: dict | list) -> Any:
    """
    Recursively remove None values from a dictionary.
    """
    if isinstance(input_data, dict):
        data = {}
        for k, v in list(input_data.items()):
            if v is None:
                continue

            if isinstance(v, (dict, list)):
                data[k] = remove_none_values(v)

            else:
                data[k] = v

        return data

    if isinstance(input_data, list):
        values = []
        for i in input_data:
            if i is None:
                continue
            if isinstance(i, (dict, list)):
                values.append(remove_none_values(i))
            else:
                values.append(i)
        return values

    return input_data


def apply_key_mapping(data: dict | list, mapping: dict) -> dict | list:
    if not isinstance(data, (dict, list)):
        raise TypeError(f"Unexpected type received: {type(data)}")

    if isinstance(data, dict):
        rslt = {}
        for key, val in data.items():
            key = mapping.get(key, key)  # noqa: PLW2901

            if isinstance(val, (dict, list)):
                rslt[key] = apply_key_mapping(val, mapping)
            else:
                rslt[key] = val
        return rslt

    if isinstance(data, list):
        rslt = []
        for val in data:
            if isinstance(val, (dict, list)):
                rslt.append(apply_key_mapping(val, mapping))
            else:
                rslt.append(val)
        return rslt

    return data
