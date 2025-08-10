#!/usr/bin/env python3
"""
Benchmark suite comparing the best implementations:
- File loading: Rust (4.38x faster than Python)
- Key mapping: Rust (4.14x faster than Python)
"""

from __future__ import annotations

import json
import pprint as pp
import statistics
import sys
import time
import zipfile
from pathlib import Path

import xmltodict
from deepdiff import DeepDiff

# from openspeleo_core import _cython_lib  # Cython implementation for key mapping
# Import the best implementations
from openspeleo_core import _rust_lib  # Rust implementation for key mapping
from openspeleo_core import ariane_core  # Rust implementation for file loading
from openspeleo_core.legacy import remove_none_values  # Base Python implementation
from scipy import stats

# ruff: noqa: T201, T203, PLR0915

# Test files
TEST_FILES = [
    "tests/artifacts/hand_survey.tml",
    "tests/artifacts/test_simple.mini.tml",
    "tests/artifacts/test_simple.tml",
    "tests/artifacts/test_with_walls.tml",
    "tests/artifacts/test_large.tml",
]


def benchmark_function(func, *args, runs=30, warmup=20, **kwargs):
    """Benchmark a function with warmup runs."""
    # Warmup runs
    for _ in range(warmup):
        func(*args, **kwargs)

    # Timed runs
    times = []
    for _ in range(runs):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        times.append(time.perf_counter() - start)

    return {
        "mean": stats.trim_mean(times, proportiontocut=0.1),
        "median": statistics.median(times),
        "stdev": statistics.stdev(times) if len(times) > 1 else 0,
        "min": min(times),
        "max": max(times),
        "runs": runs,
        "total_runs": runs + warmup,
        "result": result,
    }


def python_load_ariane_tml(filepath):
    """Pure Python implementation of loading Ariane TML files."""
    with zipfile.ZipFile(filepath, "r") as zip_file:
        xml_str = zip_file.open("Data.xml", mode="r").read().decode("utf-8")
    data = xmltodict.parse(xml_str)
    return remove_none_values(data)


def rust_load_ariane_tml(filepath):
    """Best implementation: Rust for loading Ariane TML files."""
    return ariane_core.load_ariane_tml_file_to_dict(path=str(filepath))


def python_apply_key_mapping(data, mapping):
    """Pure Python implementation of key mapping."""
    if isinstance(data, dict):
        return {
            mapping.get(k, k): python_apply_key_mapping(v, mapping)
            for k, v in data.items()
        }
    if isinstance(data, list):
        return [python_apply_key_mapping(item, mapping) for item in data]
    return data


# def cython_apply_key_mapping(data, mapping):
#     """Best implementation: Cython for key mapping."""
#     return _cython_lib.apply_key_mapping(data, mapping)


def rust_apply_key_mapping(data, mapping):
    """Best implementation: Rust for key mapping."""
    return _rust_lib.mapping.apply_key_mapping(data, mapping)


def create_test_data():
    """Create test data for key mapping benchmarks."""
    test_data = {
        "Azimut": "0.0",
        "Depth": "10.0",
        "Explorer": "Ariane",
        "Nested": {"Name": "Test", "Value": 42, "Azimut": "90.0"},
        "List": [
            {"Index": i, "Azimut": f"{i}.0", "Explorer": f"Explorer{i}"}
            for i in range(100)
        ],
    }

    mapping = {"Azimut": "Bearing", "Explorer": "Diver", "Name": "Title"}

    return test_data, mapping


def save_benchmark_results(results, filename="benchmark_results.json"):
    """Save benchmark results to a JSON file."""
    with open(filename, "w") as f:  # noqa: PTH123
        json.dump(results, f, indent=2)
    print(f"\nResults saved to {filename}")


def main():
    print("OpenSpeleo Core Benchmarking Suite - Best Implementations")
    print("=" * 60)
    print(f"Python version: {sys.version}")

    results = {
        "python_info": {"version": sys.version},
        "load_ariane_tml": {},
        "apply_key_mapping": {},
    }

    # Initial validation
    print("\n0. Validating implementations with demo.tml...")
    print("-" * 60)
    filepath = "tests/artifacts/demo.tml"
    py_result = python_load_ariane_tml(filepath)
    rust_result = rust_load_ariane_tml(filepath)

    ddiff = DeepDiff(py_result, rust_result, ignore_order=True)
    if ddiff != {}:
        print("\n=================== PYTHON ====================")
        pp.pprint(py_result)
        print("=================== RUST ====================")
        pp.pprint(rust_result)
        print("================================================\n")

        raise AssertionError(pp.pformat(ddiff, indent=2, sort_dicts=True))
    print("Done ...")

    # sys.exit(0)

    # Benchmark 1: Loading Ariane TML files
    print("\n1. Benchmarking Ariane TML file loading...")
    print("-" * 60)
    for filepath in TEST_FILES:
        filename = Path(filepath).name

        print(f"\nTesting {filename}:\n")

        print("\tValidating loading ...")
        py_result = python_load_ariane_tml(filepath)
        rust_result = rust_load_ariane_tml(filepath)

        ddiff = DeepDiff(py_result, rust_result, ignore_order=True)
        assert ddiff == {}, pp.pformat(ddiff, indent=2, sort_dicts=True)
        print("\tValidation done ...\n")

        # Python benchmark
        print("\tPython implementation...", end=" ", flush=True)
        py_result = benchmark_function(python_load_ariane_tml, filepath)
        print(f"Mean: {py_result['mean']:.6f}s")

        # Rust benchmark (best implementation)
        print("\tRust implementation...", end=" ", flush=True)
        rust_result = benchmark_function(rust_load_ariane_tml, filepath)
        print(f"Mean: {rust_result['mean']:.6f}s")

        # Calculate speedup
        speedup = py_result["mean"] / rust_result["mean"]
        print(f"\tSpeedup: {speedup:.2f}x faster")

        results["load_ariane_tml"][filename] = {
            "python": py_result,
            "rust": rust_result,
            "speedup": speedup,
        }

    # Benchmark 2: Key mapping
    print("\n2. Benchmarking key mapping...")
    print("-" * 60)

    data = python_load_ariane_tml("tests/artifacts/test_large.tml")

    mapping = {
        "TensionCorridor": "tension_corridor",
        "TensionProfile": "tension_profile",
        "angle": "angle",
        "length": "norm",
        "RadiusVector": "radius_vector",
        "RadiusCollection": "radius_collection",
        "hasProfileAzimut": "has_profile_azimuth",
        "hasProfileTilt": "has_profile_tilt",
        "profileAzimut": "profile_azimuth",
        "profileTilt": "profile_tilt",
        "dashScale": "dash_scale",
        "fillColorString": "fill_color_string",
        "lineType": "line_type",
        "lineTypeScale": "line_type_scale",
        "opacity": "opacity",
        "sizeMode": "size_mode",
        "strokeColorString": "stroke_color_string",
        "strokeThickness": "stroke_thickness",
        "constant": "constant",
        "locked": "locked_layer",
        "name": "layer_name",
        "style": "style",
        "visible": "visible",
        "layerList": "layer_list",
        "Azimut": "azimuth",
        "ClosureToID": "closure_to_id",
        "Color": "color",
        "Comment": "shot_comment",
        "Depth": "depth",
        "DepthIn": "depth_in",
        "Excluded": "excluded",
        "FromID": "from_id",
        "ID": "shot_id",
        "Inclination": "inclination",
        "Latitude": "latitude",
        "Length": "length",
        "Locked": "locked",
        "Longitude": "longitude",
        "Name": "shot_name",
        "Profiletype": "profiletype",
        "Shape": "shape",
        "Type": "shot_type",
        "Left": "left",
        "Right": "right",
        "Up": "up",
        "Down": "down",
        "Section": "section_name",
        "Date": "date",
        "Explorer": "explorers",
        "Surveyor": "surveyors",
        "SurveyData": "shots",
        "speleodb_id": "speleodb_id",
        "caveName": "cave_name",
        "unit": "unit",
        "firstStartAbsoluteElevation": "first_start_absolute_elevation",
        "useMagneticAzimuth": "use_magnetic_azimuth",
        "Layers": "ariane_viewer_layers",
        "CartoEllipse": "carto_ellipse",
        "CartoLine": "carto_line",
        "CartoLinkedSurface": "carto_linked_surface",
        "CartoOverlay": "carto_overlay",
        "CartoPage": "carto_page",
        "CartoRectangle": "carto_rectangle",
        "CartoSelection": "carto_selection",
        "CartoSpline": "carto_spline",
        "Constraints": "constraints",
        "ListAnnotation": "list_annotation",
        "Data": "data",
    }

    print("\n\tValidating key mapping ...")
    print('\t\t- Executing "python_apply_key_mapping"...')
    py_result = python_apply_key_mapping(data, mapping)
    print('\t\t- Executing "rust_apply_key_mapping"...')
    rust_result = rust_apply_key_mapping(data, mapping)
    ddiff = DeepDiff(py_result, rust_result, ignore_order=True)
    assert ddiff == {}, pp.pformat(ddiff, indent=2, sort_dicts=True)
    print("\tValidation done ...\n")

    # Python benchmark
    print("\tPython implementation...", end=" ", flush=True)
    py_map_result = benchmark_function(python_apply_key_mapping, data, mapping)
    print(f"Mean: {py_map_result['mean']:.6f}s")

    # Rust benchmark
    print("\tRust implementation...", end=" ", flush=True)
    rust_map_result = benchmark_function(rust_apply_key_mapping, data, mapping)
    print(f"Mean: {rust_map_result['mean']:.6f}s")

    # Calculate speedup
    map_speedup = py_map_result["mean"] / rust_map_result["mean"]
    print(f"\tSpeedup: {map_speedup:.2f}x faster")

    results["apply_key_mapping"] = {
        "python": py_map_result,
        "rust": rust_map_result,
        "speedup": map_speedup,
    }

    # Overall summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)

    print("\nFile Loading Speedups (Rust vs Python):")
    for filename, data in results["load_ariane_tml"].items():
        print(f"  {filename:30} {data['speedup']:.2f}x")

    avg_load_speedup = statistics.mean(
        [data["speedup"] for data in results["load_ariane_tml"].values()]
    )
    print(f"\n  Average loading speedup: {avg_load_speedup:.2f}x")

    print(
        "\nKey Mapping Speedup (Rust vs Python): "
        f"{results['apply_key_mapping']['speedup']:.2f}x"
    )

    # Performance characteristics
    print("\n" + "=" * 60)
    print("Performance Consistency")
    print("=" * 60)

    print("\nFile Loading (Rust):")
    for filename, data in results["load_ariane_tml"].items():
        rust_data = data["rust"]
        consistency = (
            (rust_data["stdev"] / rust_data["mean"]) * 100
            if rust_data["mean"] > 0
            else 0
        )
        print(f"  {filename:30} {consistency:.1f}% variation")

    map_data = results["apply_key_mapping"]["rust"]
    consistency = (
        (map_data["stdev"] / map_data["mean"]) * 100 if map_data["mean"] > 0 else 0
    )
    print(f"\nKey Mapping (Rust): {consistency:.1f}% variation")

    # Save results
    save_benchmark_results(results)


if __name__ == "__main__":
    main()
