# Plan: ignore Rust build artifacts

1. Update `.gitignore` to ignore Cargo build output (`target/`).
2. Verify Git no longer reports Rust build artifacts as untracked after cargo commands.
3. Commit and push as a small hygiene fix without force-push.
