use std::{collections::BTreeSet, fs, path::Path};

use instagram_private_api_rust::models::generated::{RESPONSE_MODULES, TYPE_MODULES};

fn ts_stems(dir: &Path) -> BTreeSet<String> {
    fs::read_dir(dir)
        .expect("directory exists")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? != "ts" {
                return None;
            }
            Some(
                path.file_stem()?
                    .to_string_lossy()
                    .replace(['.', '-'], "_"),
            )
        })
        .collect()
}

#[test]
fn response_manifest_matches_typescript_response_declarations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected = ts_stems(&root.join("src/responses"));
    let actual: BTreeSet<String> = RESPONSE_MODULES.iter().map(|s| (*s).to_string()).collect();

    assert_eq!(actual, expected, "response manifest is out-of-sync");
}

#[test]
fn type_manifest_matches_typescript_type_declarations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected = ts_stems(&root.join("src/types"));
    let actual: BTreeSet<String> = TYPE_MODULES.iter().map(|s| (*s).to_string()).collect();

    assert_eq!(actual, expected, "type manifest is out-of-sync");
}
