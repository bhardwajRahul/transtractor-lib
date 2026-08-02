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

fn validate_spec_file_name(spec_path: &Path, spec: &Spec) -> Result<(), String> {
    let file_stem = spec_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("{}: missing or invalid file stem", spec_path.display()))?;

    let components: Vec<&str> = file_stem.split("__").collect();
    if components.len() != 5 {
        return Err(format!(
            "{}: expected exactly 5 filename components separated by double underscores, found {} in {:?}",
            spec_path.display(),
            components.len(),
            components
        ));
    }

    let key_components: Vec<&str> = spec
        .statement_data
        .key
        .as_deref()
        .ok_or_else(|| format!("{}: statement_data.key is missing", spec_path.display()))?
        .split("__")
        .collect();

    if key_components.len() != 4 {
        return Err(format!(
            "{}: expected statement_data.key to have exactly 4 components separated by double underscores, found {} in {:?}",
            spec_path.display(),
            key_components.len(),
            key_components
        ));
    }

    let expected_dir = key_components[0];
    let actual_dir = spec_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{}: missing parent directory name", spec_path.display()))?;

    if actual_dir != expected_dir {
        return Err(format!(
            "{}: expected to be in directory {:?} based on statement_data.key {:?}, found {:?}",
            spec_path.display(),
            expected_dir,
            spec.statement_data.key,
            actual_dir
        ));
    }

    if components[0] != key_components[1]
        || components[1] != key_components[2]
        || components[2] != key_components[3]
    {
        return Err(format!(
            "{}: filename components {:?} do not match statement_data.key components {:?} as required",
            spec_path.display(),
            components,
            key_components
        ));
    }

    if components[4].parse::<i64>().is_err() {
        return Err(format!(
            "{}: fifth filename component {:?} is not a valid integer",
            spec_path.display(),
            components[4]
        ));
    }

    Ok(())
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

        if let Err(error) = validate_spec_file_name(&spec_path, &spec) {
            failures.push(error);
            continue;
        }

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
