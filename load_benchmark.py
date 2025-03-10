import statistics
import time
import zipfile

from deepdiff import DeepDiff

from openspeleo_core import load_ariane_tml_file_to_dict
from openspeleo_core import xml_str_to_dict

if __name__ == "__main__":
    for filepath in [
        "tests/artifacts/hand_survey.tml",
        "tests/artifacts/test_large.tml",
        "tests/artifacts/test_simple.mini.tml",
        "tests/artifacts/test_simple.tml",
        "tests/artifacts/test_with_walls.tml",
    ]:
        print(f"\nFilename: {filepath} ...\n")  # noqa: T201

        runs = []
        for idx in range(15):
            start_t = time.perf_counter()
            core_data = load_ariane_tml_file_to_dict(path=str(filepath))["CaveFile"]
            runs.append(time.perf_counter() - start_t)
            print(f"[{idx + 1:02d}] [Export] Elapsed: {runs[-1]:.2f} secs")  # noqa: T201
        print(f"Average: {statistics.mean(runs[5:]):.2f} secs")  # noqa: T201

        with zipfile.ZipFile(filepath, "r") as zip_file:
            xml_str = zip_file.open("Data.xml", mode="r").read().decode("utf-8")
        expected_data = xml_str_to_dict(xml_str)["CaveFile"]

        diff = DeepDiff(core_data, expected_data, ignore_order=True)
        assert diff == {}, f"Round trip transformation failed: {diff}"
