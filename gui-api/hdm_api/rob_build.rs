use std::fs;

fn main() {
    let cargo_manifest = "Cargo.toml";
    let cargo_toml = fs::read_to_string(cargo_manifest).unwrap();

    let mut new_toml = String::new();
    for line in cargo_toml.lines() {
        if line.trim_start().starts_with("crate-type") {
            continue;
        }
        new_toml.push_str(line);
        new_toml.push('\n');
    }

    // Rust için rlib + staticlib
    new_toml.push_str("crate-type = [\"rlib\", \"staticlib\"]\n");

    fs::write(cargo_manifest, new_toml).unwrap();
    println!("cargo:warning=Configured hdm_api for Rust build");
}
