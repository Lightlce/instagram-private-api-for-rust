# Dual-run rollout validation

This repository now contains a dual-run validation script at:

- `rust/scripts/dual_run_validation.sh`

The script executes:

1. `cargo test`
2. `npm test -- --runInBand`

Use this during staged rollout to compare behavior while Rust and TypeScript implementations coexist.
