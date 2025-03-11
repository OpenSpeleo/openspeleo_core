from openspeleo_core._lib import mapping as _mapping


def apply_key_mapping(data: dict | list, mapping: dict) -> dict:
    if not isinstance(data, (dict, list)):
        raise TypeError(f"Unexpected type received for `data`: {type(data)}")

    if not isinstance(mapping, dict):
        raise TypeError(f"Unexpected type received for `mapping`: {type(mapping)}")

    return _mapping.apply_key_mapping(data=data, mapping=mapping)
