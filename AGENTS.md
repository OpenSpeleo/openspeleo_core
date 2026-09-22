# openspeleo_core Agent Instructions

## Project overview

`openspeleo_core` provides Rust-backed cave-survey parsing and conversion
through PyO3 and a Python package built with Maturin.

- `src_rust/lib.rs` and `mapping.rs`: native module exports and key mapping.
- `src_rust/ariane/`: Ariane loading, serialization, and deserialization.
- `src_python/openspeleo_core/`: Python wrappers and type stubs.
- `src_rust/bin/stub_gen.rs`: generation of Python type stubs.
- `tests/`: Python integration tests for native behavior.

Preserve Python signatures, exception behavior, format compatibility, and the
native module name `openspeleo_core._rust_lib`. Update binding code and type
stubs together when the public API changes; test through the Python boundary.

## Temporary agent files

Keep agent plans, task lists, TODO tracking, progress notes, review notes, and
scratch lessons outside the repository tree, including all submodules. Use a
unique task directory under `/tmp/` (for example, create one with
`mktemp -d /tmp/speleodb-task.XXXXXX`) or another OS temporary directory whose
resolved path is outside every checkout.

Never create or update these working files inside the checkout, even in ignored
directories such as `tasks/`, `todos/`, or `plans/`. Never stage or commit them.
Existing tracked task and lesson files are historical references; do not append
new work to them. Keep durable product and architecture documentation in
`docs/`, without embedding task checklists or linking to temporary files. Before
an authorized commit, inspect the staged filenames and exclude all agent working
files.

## Working rules

- Read the relevant source, tests, and configuration before editing. Keep
  changes focused and preserve public behavior unless the task requires changing
  it.
- Inspect staged and unstaged changes separately before and after work. Preserve
  unrelated user work, existing branches, and submodule revisions.
- Do not stage, commit, push, open a PR, tag, or publish without explicit user
  authorization. Never install Git hooks or configure `core.hooksPath`.
- Run commands from this repository's root unless stated otherwise. In the
  monorepo, also follow the integration root's `AGENTS.md`; keep this repository
  usable independently and keep product changes in their owning repository.
- For documentation-only changes, run `prek run prettier --files AGENTS.md` and
  whitespace checks. For behavior changes, run focused regression tests and the
  relevant full suite. Report commands, results, and any missing prerequisites.

## Python development and verification

Python support and dependencies are defined in `pyproject.toml`; the current
minimum is Python 3.11 and CI covers Python 3.11 through 3.14. Keep code
compatible with that range. Follow the configured Ruff rules, use existing
package constants and helpers, and add regression coverage at the affected
parser, model, or command boundary.

Run the standalone checks from this package directory:

```bash
uv sync --frozen --all-extras --dev
uv run ruff check .
uv run ruff format --check .
uv run pytest
uv run prek run --all-files
```

Prek may apply fixes; inspect its diff and rerun affected checks. For test runs,
prefer the CI command `uv run pytest` over legacy Make targets that may require
extra tools or perform cleanup. Preserve test fixtures; put generated test
outputs in temporary directories.

## Dependencies and integration

Keep this package's `pyproject.toml` and `uv.lock` authoritative for standalone
use. Run `uv lock` here after dependency changes. When working in the monorepo,
also refresh the root integration lock and validate the root frozen sync as
directed by its `AGENTS.md`. Do not add a uv workspace or monorepo-only paths to
standalone dependency declarations.

## Native builds

Keep this Cargo project independent; do not add it to a root Cargo workspace.
Preserve `Cargo.lock`, `uv.lock`, and the `src_python` Maturin layout. Editable
builds use the explicit `dev` profile. uv cache keys must continue to include
`pyproject.toml`, `Cargo.toml`, `Cargo.lock`, and `src_rust/**/*`.

For Rust changes, also run the checks used by the standalone CI:

```bash
cargo fmt -- --check
cargo clippy --locked -- -D warnings
cargo check --locked
cargo test --locked --all
uv run maturin build --locked --release
```

When bindings change, regenerate stubs using `make stubs`, inspect the generated
diff, and rerun Python tests. In the monorepo, also run `make check-rust` and
`make build-core` from the integration root for native changes.

Do not use `make clean` or `make build` for routine validation: the current
clean recipe deletes lockfiles and stubs, and the build target also updates
dependencies. Use the direct build command above to preserve pinned inputs.
