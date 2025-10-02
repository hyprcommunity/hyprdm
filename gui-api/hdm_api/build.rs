use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let choice = env::var("HDM_API_LIB_TYPE").unwrap_or_else(|_| "rust".to_string());
    println!("cargo:warning=HDM_API_LIB_TYPE = {}", choice);

    // 1️⃣ hdm_api/Cargo.toml düzenle
    let cargo_toml_path = Path::new("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(cargo_toml_path)
        .expect("Failed to read hdm_api/Cargo.toml");

    let mut new_lines = Vec::new();
    let mut in_lib_section = false;
    let mut has_crate_type = false;

    for line in cargo_toml_content.lines() {
        let trimmed = line.trim();

        if trimmed == "[lib]" {
            in_lib_section = true;
            new_lines.push(line.to_string());
            continue;
        }

        if in_lib_section && trimmed.starts_with("crate-type") {
            has_crate_type = true;

            if choice == "c" {
                // crate-type değerlerini ayıkla
                let mut types: Vec<&str> = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .replace(['[', ']', '"'], "")
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty() && *s != "staticlib") // staticlib'i kaldır
                    .collect();

                if types.is_empty() {
                    // boş kalmasın, default olarak cdylib bırak
                    types.push("cdylib");
                }

                let new_line = format!("crate-type = [{}]", types
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", "));

                new_lines.push(new_line);
                continue;
            }
        }

        if in_lib_section && trimmed.starts_with('[') {
            in_lib_section = false;
        }

        new_lines.push(line.to_string());
    }

    // Rust modunda crate-type yoksa ekle
    if choice == "rust" && !has_crate_type {
        let mut updated_lines = Vec::new();
        for line in new_lines {
            updated_lines.push(line.clone());
            if line.trim() == "[lib]" {
                updated_lines.push("crate-type = [\"staticlib\"]".to_string());
            }
        }
        new_lines = updated_lines;
    }

    fs::write(cargo_toml_path, new_lines.join("\n"))
        .expect("Failed to write hdm_api/Cargo.toml");

    // 2️⃣ wlrootbackends dokunulmaz
    println!("cargo:warning=wlrootbackends will build with existing hdm_api configuration");
}
