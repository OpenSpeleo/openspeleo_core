# ruff: noqa: E402

from __future__ import annotations

import warnings

# Suppress deprecation warnings from ptr package
warnings.filterwarnings("ignore", category=DeprecationWarning, module="ptr")
warnings.filterwarnings("ignore", category=UserWarning, module="ptr")

from Cython.Build import cythonize
from setuptools import Extension
from setuptools import setup

ext_modules = [
    Extension(
        "openspeleo_core._cython_lib",
        ["src_cython/_cython_lib.pyx"],
        extra_compile_args=[
            "-O3",  # Maximum optimization
            "-march=native",  # CPU-specific optimizations
            "-mtune=native",  # CPU-specific tuning
            "-ffast-math",  # Fast floating point (less precise)
            "-funroll-loops",  # Loop unrolling
            "-ftree-vectorize",  # Auto-vectorization
            "-fomit-frame-pointer",  # Faster function calls
            "-flto",  # Link-time optimization
        ],
        define_macros=[],
    )
]

setup(
    ext_modules=cythonize(
        ext_modules,
        build_dir="build",  # This is the key: specify your build directory
        compiler_directives={
            "language_level": "3",
            "boundscheck": False,
            "wraparound": False,
            "cdivision": True,
            "initializedcheck": False,
            "nonecheck": False,
            "overflowcheck": False,
            "embedsignature": True,
            "profile": False,
            "linetrace": False,
            "infer_types": True,
            "optimize.use_switch": True,
            "optimize.unpack_method_calls": True,
        },
    ),
    packages=[],  # Explicitly set no packages to avoid auto-discovery
    py_modules=[],  # No Python modules either
)
