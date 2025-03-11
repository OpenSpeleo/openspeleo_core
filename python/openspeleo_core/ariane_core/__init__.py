from pathlib import Path
from typing import Any

from openspeleo_core._lib import ariane as _ariane  # type: ignore  # noqa: PGH003

dict_to_xml_str = _ariane.dict_to_xml_str
load_ariane_tml_file_to_dict = _ariane.load_ariane_tml_file_to_dict


def xml_str_to_dict(xml_str: str | Path, keep_null: bool = True) -> Any:
    return _ariane.xml_str_to_dict(str(xml_str, keep_null))
