use std::env;
use std::fs;

fn main() {
    let choice = env::var("HDM_API_LIB_TYPE").unwrap_or_else(|_| {
        // Default: Rust backend
        "rust".to_string()
    });

    let hdm_cargo_path = "Cargo.toml";
    let mut hdm_cargo = fs::read_to_string(hdm_cargo_path)
        .expect("Failed to read hdm_api/Cargo.toml");

    match choice.as_str() {
        "c" => {
            // FFI C shared library -> comment out crate-type
            hdm_cargo = hdm_cargo.replace("#crate-type = [\"staticlib\"]", "#crate-type = [\"staticlib\"]");
            println!("cargo:warning=Using C shared library (FFI)");
        }
        "rust" => {
            // Rust backend library -> uncomment crate-type
            hdm_cargo = hdm_cargo.replace("crate-type = [\"staticlib\"]", "crate-type = [\"staticlib\"]");
            println!("cargo:warning=Using Rust backend library");
        }
        _ => {
            panic!("Invalid choice: {}. HDM_API_LIB_TYPE must be 'c' or 'rust'", choice);
        }
    }

    fs::write(hdm_cargo_path, hdm_cargo).expect("Failed to write Cargo.toml");
}
