from pathlib import Path

from openspeleo_core._rust_lib import ariane as _ariane  # type: ignore  # noqa: PGH003


def load_ariane_tml_file_to_dict(path: str | Path) -> dict:
    if not Path(path).exists():
        raise FileNotFoundError(f"Impossible to find {path} ...")

    return _ariane.load_ariane_tml_file_to_dict(str(path))


def xml_str_to_dict(xml_str: str, keep_null: bool = True) -> dict:
    return _ariane.xml_str_to_dict(xml_str, keep_null)


def dict_to_xml_str(data: dict, root_name: str) -> str:
    return _ariane.dict_to_xml_str(data, root_name)


def load_ariane_tml_file_to_json(path: str | Path) -> str:
    if not Path(path).exists():
        raise FileNotFoundError(f"Impossible to find {path} ...")

    return _ariane.load_ariane_tml_file_to_json(str(path))


def xml_str_to_json(xml_str: str, keep_null: bool = True) -> str:
    return _ariane.xml_str_to_json(xml_str, keep_null)
