from openspeleo_core import _cython_lib


def apply_key_mapping(data: dict | list, mapping: dict[str, str]) -> dict:
    if not isinstance(data, (dict, list)):
        raise TypeError(f"Unexpected type received for `data`: {type(data)}")

    if not isinstance(mapping, dict):
        raise TypeError(f"Unexpected type received for `mapping`: {type(mapping)}")

    return _cython_lib.apply_key_mapping(data=data, mapping=mapping)
