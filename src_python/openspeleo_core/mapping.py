from openspeleo_core._rust_lib import mapping as _lib  # type: ignore  # noqa: PGH003


def apply_key_mapping(data: dict | list, mapping: dict[str, str]) -> dict:
    if not isinstance(data, (dict, list)):
        raise TypeError(f"Unexpected type received for `data`: {type(data)}")

    if not isinstance(mapping, dict):
        raise TypeError(f"Unexpected type received for `mapping`: {type(mapping)}")

    return _lib.apply_key_mapping(data=data, mapping=mapping)
