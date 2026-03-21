#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT_DIR"

echo "== Rust checks =="
cargo test

echo "== TypeScript checks =="
npm test -- --runInBand

echo "Dual-run validation complete."
