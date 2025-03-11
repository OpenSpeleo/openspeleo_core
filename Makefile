.PHONY: clean test coverage build install lint

# ============================================================================ #
# CLEAN COMMANDS
# ============================================================================ #

clean: clean-build clean-pyc ## remove all build, test, coverage and Python artifacts

clean-build: ## remove build artifacts
	rm -rf .venv/
	rm -rf dist/
	rm -rf build/
	rm -rf target/
	rm -f uv.lock
	uv clean

clean-pyc: ## remove Python file artifacts
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +
	find . -name '__pycache__' -exec rm -fr {} +

# ============================================================================ #
# INSTALL COMMANDS
# ============================================================================ #

develop: clean ## install the package to the active Python's site-packages
	maturin develop --uv

build: clean ## install the package to the active Python's site-packages
	maturin build --release

stubs: ## generate *.pyi stubs file
	cargo run --bin stub_gen
