use std::fs;

fn main() {
    let cargo_manifest = "Cargo.toml";
    let cargo_toml = fs::read_to_string(cargo_manifest)
        .expect("Failed to read Cargo.toml");

    let mut new_lines = Vec::new();
    let mut removed = false;

    for line in cargo_toml.lines() {
        let trimmed = line.trim_start();

        // Sadece staticlib tanımı varsa kaldır
        if trimmed.starts_with("crate-type")
            && trimmed.contains("staticlib")
        {
            removed = true;
            continue;
        }

        new_lines.push(line.to_string());
    }

    fs::write(cargo_manifest, new_lines.join("\n") + "\n")
        .expect("Failed to write Cargo.toml");

    if removed {
        println!("cargo:warning=Configured hdm_api for C/FFI build (staticlib removed)");
    } else {
        println!("cargo:warning=No staticlib entry found in Cargo.toml (already clean for C/FFI)");
    }
}
