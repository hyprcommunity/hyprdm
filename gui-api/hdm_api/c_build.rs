use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:warning=Building hdm_api in C/FFI mode (staticlib removed)");

    let cargo_toml_path = Path::new("Cargo.toml");
    let content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    let mut new_lines = Vec::new();
    let mut in_lib_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[lib]" {
            in_lib_section = true;
            new_lines.push(line.to_string());
            continue;
        }

        if in_lib_section {
            if trimmed.starts_with("crate-type") {
                // C modunda staticlib satırını kaldır
                continue;
            } else if trimmed.starts_with('[') {
                in_lib_section = false;
            }
        }

        new_lines.push(line.to_string());
    }

    fs::write(cargo_toml_path, new_lines.join("\n")).expect("Failed to write Cargo.toml");

    println!("cargo:warning=wlrootbackends will build and link hdm_api crate in C/FFI mode");
}
