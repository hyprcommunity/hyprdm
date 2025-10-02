use std::fs;

fn main() {
    let cargo_manifest = "Cargo.toml";
    let cargo_toml = fs::read_to_string(cargo_manifest).unwrap();

    let mut lines: Vec<String> = cargo_toml
        .lines()
        .filter(|line| !line.trim_start().starts_with("crate-type"))
        .map(|l| l.to_string())
        .collect();

    let mut new_toml = String::new();
    let mut added = false;

    for line in &lines {
        new_toml.push_str(line);
        new_toml.push('\n');
        if line.trim() == "[lib]" && !added {
            new_toml.push_str("crate-type = [\"staticlib\"]\n");
            added = true;
        }
    }

    fs::write(cargo_manifest, new_toml).unwrap();
    println!("cargo:warning=Configured hdm_api for Rust build (crate-type = [\"staticlib\"])");
}
