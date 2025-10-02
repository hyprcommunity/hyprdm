use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let choice = env::var("HDM_API_LIB_TYPE").unwrap_or_else(|_| "rust".to_string());

    // 1️⃣ hdm_api/Cargo.toml düzenle
    let cargo_toml_path = Path::new("Cargo.toml");
    let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Failed to read hdm_api/Cargo.toml");

    let mut new_lines = Vec::new();
    let mut in_lib_section = false;
    let mut has_crate_type = false;

    for line in cargo_toml.lines() {
        let trimmed = line.trim();

        if trimmed == "[lib]" {
            in_lib_section = true;
            new_lines.push(line.to_string());
            continue;
        }

        if in_lib_section {
            if trimmed.starts_with("crate-type") {
                has_crate_type = true;
                // Rust modunda bırak, C/FFI modunda kaldır
                if choice == "c" {
                    continue; // satırı at
                }
            } else if trimmed.starts_with("[") {
                in_lib_section = false;
            }
        }

        new_lines.push(line.to_string());
    }

    // Rust modunda crate-type yoksa ekle
    if choice == "rust" && !has_crate_type {
        let mut updated_lines = Vec::new();
        let mut added = false;
        for line in new_lines {
            updated_lines.push(line.clone());
            if line.trim() == "[lib]" && !added {
                updated_lines.push("crate-type = [\"staticlib\"]".to_string());
                added = true;
            }
        }
        new_lines = updated_lines;
    }

    fs::write(cargo_toml_path, new_lines.join("\n")).expect("Failed to write hdm_api/Cargo.toml");

    println!("cargo:warning=HDM_API_LIB_TYPE = {}", choice);

    // 2️⃣ wlrootbackends için feature kontrolü
    let wlroot_toml = Path::new("wlrootbackends/Cargo.toml");
    if wlroot_toml.exists() {
        let wl_content = fs::read_to_string(wlroot_toml).expect("Failed to read wlrootbackends/Cargo.toml");
        let mut wl_lines = Vec::new();
        for line in wl_content.lines() {
            if line.contains("hdm_api") {
                if choice == "c" {
                    continue;
                }
            }
            wl_lines.push(line.to_string());
        }
        fs::write(wlroot_toml, wl_lines.join("\n")).expect("Failed to write wlrootbackends/Cargo.toml");
        if choice == "c" {
            println!("cargo:warning=wlrootbackends will build without hdm_api for C/FFI mode");
        }
    }
}
