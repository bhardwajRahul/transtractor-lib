use std::fs;
use std::path::{Path, PathBuf};
use transtractor::configs::db::ConfigDB;
use transtractor::structs::Spec;

fn collect_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read directory {}: {}", dir.display(), e));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| {
            panic!(
                "Failed to read an entry in directory {}: {}",
                dir.display(),
                e
            )
        });
        let path = entry.path();

        if path.is_dir() {
            collect_json_files(&path, out);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            out.push(path);
        }
    }
}

#[test]
fn all_spec_files_valid() {
    let fixtures_root = Path::new("tests/fixtures/spec");
    assert!(
        fixtures_root.exists(),
        "Spec fixtures directory does not exist: {}",
        fixtures_root.display()
    );

    let mut spec_files = Vec::new();
    collect_json_files(fixtures_root, &mut spec_files);
    spec_files.sort();

    assert!(
        !spec_files.is_empty(),
        "No JSON spec fixtures found under {}",
        fixtures_root.display()
    );

    let config_db = ConfigDB::new();
    let mut failures = Vec::new();

    for spec_path in spec_files {
        let spec_content = match fs::read_to_string(&spec_path) {
            Ok(content) => content,
            Err(error) => {
                failures.push(format!(
                    "{}: failed to read file: {}",
                    spec_path.display(),
                    error
                ));
                continue;
            }
        };

        let spec = match Spec::from_json(&spec_content) {
            Ok(spec) => spec,
            Err(error) => {
                failures.push(format!(
                    "{}: failed to parse JSON spec: {}",
                    spec_path.display(),
                    error
                ));
                continue;
            }
        };

        if let Err(error) = spec.validate(&config_db) {
            failures.push(format!("{}:\n{}", spec_path.display(), error));
        }
    }

    assert!(
        failures.is_empty(),
        "{} spec validation failure(s):\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}
