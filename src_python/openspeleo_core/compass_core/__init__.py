# -*- coding: utf-8 -*-

from typing import Any

from openspeleo_core import _rust_lib


def convert_xls_json_to_dat(survey: dict[str, Any]) -> bytes:
    return _rust_lib.compass.convert_xls_json_to_dat(survey)
