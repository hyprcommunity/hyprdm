use std::fs;

fn main() {
    let cargo_manifest = "Cargo.toml";
    let cargo_toml = fs::read_to_string(cargo_manifest).unwrap();

    let new_toml: String = cargo_toml
        .lines()
        .filter(|line| !line.trim_start().starts_with("crate-type"))
        .map(|line| format!("{}\n", line))
        .collect();

    fs::write(cargo_manifest, new_toml).unwrap();
    println!("cargo:warning=Configured hdm_api for C/FFI build (crate-type removed)");
}
