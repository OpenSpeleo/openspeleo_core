import argparse
import json
import logging
import pathlib

from openspeleo_core import ariane_core

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)


def convert(args):
    parser = argparse.ArgumentParser(
        prog="convert", description="Convert a Survey File"
    )
    parser.add_argument(
        "-i",
        "--input_file",
        type=pathlib.Path,
        required=True,
        help="Path to the TML file to be validated",
    )

    parser.add_argument(
        "-o",
        "--output_file",
        type=pathlib.Path,
        default=None,
        required=True,
        help="Path to save the converted file at.",
    )

    parser.add_argument(
        "-w",
        "--overwrite",
        action="store_true",
        help="Allow overwrite an already existing file.",
        default=False,
    )

    parser.add_argument(
        "-f",
        "--format",
        type=str,
        choices=["json"],
        required=True,
        help="Conversion format used.",
    )

    parsed_args = parser.parse_args(args)

    input_file: pathlib.Path = parsed_args.input_file
    output_file: pathlib.Path = parsed_args.output_file

    if not input_file.exists():
        raise FileNotFoundError(f"File not found: `{input_file}`")

    if output_file.exists() and not parsed_args.overwrite:
        raise FileExistsError(
            f"The file `{output_file}` already existing. "
            "Please pass the flag `--overwrite` to ignore."
        )

    match input_file.suffix:
        case ".tml":
            data: dict = ariane_core.load_ariane_tml_file_to_dict(input_file)
        case _:
            raise ValueError(f"Unsupported file format: `{input_file.suffix}`")

    with output_file.open(mode="w") as fp:
        json.dump(data, fp, indent=2, sort_keys=True)
