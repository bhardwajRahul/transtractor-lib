use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    println!("cargo:rerun-if-changed={}", dir.display());
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("Failed to read directory {}: {error}", dir.display()));

    for entry in entries {
        let entry = entry.unwrap_or_else(|error| {
            panic!(
                "Failed to read an entry in directory {}: {error}",
                dir.display()
            )
        });
        let path = entry.path();

        if path.is_dir() {
            collect_json_files(&path, out);
        } else if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            println!("cargo:rerun-if-changed={}", path.display());
            out.push(path);
        }
    }
}

fn sanitise_test_name(value: &str) -> String {
    let mut sanitized = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            sanitized.push(character.to_ascii_lowercase());
        } else if !sanitized.ends_with('_') {
            sanitized.push('_');
        }
    }
    sanitized.trim_matches('_').to_string()
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let fixtures_root = manifest_dir.join("tests/fixtures/spec");
    let mut spec_files = Vec::new();
    collect_json_files(&fixtures_root, &mut spec_files);
    spec_files.sort();

    let mut generated = String::new();
    for spec_path in &spec_files {
        let relative_path = spec_path
            .strip_prefix(&manifest_dir)
            .expect("fixture path should be under the manifest directory")
            .to_string_lossy()
            .replace('\\', "/");
        let parent_name = spec_path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("spec");
        let file_stem = spec_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("fixture");
        let parent_name = sanitise_test_name(parent_name);
        let file_stem = sanitise_test_name(file_stem);

        generated.push_str(&format!(
            "#[test]\nfn spec_{parent_name}_{file_stem}() {{\n    validate_spec_file({relative_path:?});\n}}\n\n"
        ));
    }

    let output_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("spec_tests.rs");
    fs::write(output_path, generated).expect("Failed to write generated spec tests");
}
