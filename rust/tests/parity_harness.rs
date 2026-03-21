use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    typescript: Vec<String>,
    rust: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Matrix {
    tracked_groups: Vec<Group>,
}

#[test]
fn parity_matrix_references_existing_paths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let raw = fs::read_to_string(root.join("rust/parity/matrix.json")).expect("matrix exists");
    let matrix: Matrix = serde_json::from_str(&raw).expect("matrix valid json");

    assert!(!matrix.tracked_groups.is_empty());

    for group in matrix.tracked_groups {
        assert!(!group.name.trim().is_empty());
        for path in group.typescript.iter().chain(group.rust.iter()) {
            assert!(root.join(path).exists(), "missing mapped path: {path}");
        }
    }
}
