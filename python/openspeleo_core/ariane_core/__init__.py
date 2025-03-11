from pathlib import Path

from openspeleo_core._lib import ariane as _ariane  # type: ignore  # noqa: PGH003

dict_to_xml_str = _ariane.dict_to_xml_str


def load_ariane_tml_file_to_dict(path: str | Path) -> dict:
    if not Path(path).exists():
        raise FileNotFoundError(f"Impossible to find {path} ...")

    return _ariane.load_ariane_tml_file_to_dict(str(path))


def xml_str_to_dict(xml_str: str, keep_null: bool = True) -> dict:
    return _ariane.xml_str_to_dict(xml_str, keep_null)
