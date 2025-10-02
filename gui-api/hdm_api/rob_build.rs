use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:warning=Building hdm_api in Rust mode (staticlib)");

    let cargo_toml_path = Path::new("Cargo.toml");
    let content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    let mut new_lines = Vec::new();
    let mut in_lib_section = false;
    let mut has_crate_type = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[lib]" {
            in_lib_section = true;
            new_lines.push(line.to_string());
            continue;
        }

        if in_lib_section {
            if trimmed.starts_with("crate-type") {
                has_crate_type = true;
            } else if trimmed.starts_with('[') {
                in_lib_section = false;
            }
        }

        new_lines.push(line.to_string());
    }

    // Rust modunda crate-type yoksa ekle
    if !has_crate_type {
        let mut updated_lines = Vec::new();
        for line in new_lines {
            updated_lines.push(line.clone());
            if line.trim() == "[lib]" {
                updated_lines.push("crate-type = [\"staticlib\"]".to_string());
            }
        }
        new_lines = updated_lines;
    }

    fs::write(cargo_toml_path, new_lines.join("\n")).expect("Failed to write Cargo.toml");
}
