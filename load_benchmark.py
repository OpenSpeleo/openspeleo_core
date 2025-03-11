import statistics
import time
import zipfile

import xmltodict
from deepdiff import DeepDiff
from openspeleo_core import ariane_core
from tests.test_xml_to_dict import remove_none_values

if __name__ == "__main__":
    for filepath in [
        "tests/artifacts/hand_survey.tml",
        "tests/artifacts/test_large.tml",  # LONG
        "tests/artifacts/test_simple.mini.tml",
        "tests/artifacts/test_simple.tml",
        "tests/artifacts/test_with_walls.tml",
    ]:
        print(f"\nFilename: {filepath} ...\n")  # noqa: T201

        runs = []
        for idx in range(15):
            start_t = time.perf_counter()
            core_data = ariane_core.load_ariane_tml_file_to_dict(path=str(filepath))[
                "CaveFile"
            ]
            runs.append(time.perf_counter() - start_t)
            print(f"[{idx + 1:02d}] [Export] Elapsed: {runs[-1]:.2f} secs")  # noqa: T201
        print(f"Average: {statistics.mean(runs[5:]):.2f} secs")  # noqa: T201

        with zipfile.ZipFile(filepath, "r") as zip_file:
            xml_str = zip_file.open("Data.xml", mode="r").read().decode("utf-8")
        expected_data = xmltodict.parse(xml_str)["CaveFile"]
        expected_data = remove_none_values(expected_data)

        diff = DeepDiff(core_data, expected_data, ignore_order=True)
        assert diff == {}, f"Round trip transformation failed: {diff}"
