# Rust backend scaffold (instagram-private-api-for-rust)

This folder contains the Rust migration scaffold for the TypeScript `instagram-private-api` project.

## What is implemented today

- Core module boundaries (`client`, `state`, `http`, `repositories`, `feeds`, `services`, `models`, `errors`).
- Deterministic state/device generation and JSON state persistence.
- Request-layer primitives (default headers, payload signing, retry policy, status mapping).
- Repository/service/feed scaffolds and basic parity/test tooling.

## Prerequisites

- Rust stable toolchain (recommended via `rustup`).
- Python 3 (for model manifest generation script).
- Node.js + npm (only needed for dual-run TypeScript parity validation).

## Quick start

From repository root:

```bash
# 1) format and test Rust code
cargo fmt --all
cargo test

# 2) run example flow
cargo run --example basic_flow
```

## Project layout

- `rust/src/` — Rust source modules.
- `rust/scripts/generate_models.py` — generates Rust model manifest from TS declarations.
- `rust/src/models/generated/mod.rs` — generated model manifest output.
- `rust/parity/matrix.json` — parity mapping used by parity harness tests.
- `rust/scripts/dual_run_validation.sh` — Rust + TypeScript staged validation helper.

## Regenerate model manifest

```bash
python rust/scripts/generate_models.py
cargo test
```

## Run dual-run validation

```bash
bash rust/scripts/dual_run_validation.sh
```

Notes:

- Rust phase runs `cargo test`.
- TypeScript phase runs `npm test` and requires TS test dependencies (e.g. `mocha`) to be installed.

## Current status and scope

This is an incremental migration scaffold intended for parity-driven development. Some modules are placeholders and should be expanded endpoint-by-endpoint while validating behavior against the TypeScript implementation.
