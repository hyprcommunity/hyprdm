use std::env;
use std::process::Command;

fn main() {
    let lib_type = env::var("HDM_API_LIB_TYPE").unwrap_or_else(|_| "rust".to_string());

    let script = match lib_type.as_str() {
        "c" => "c_build.rs",
        _   => "rob_build.rs",
    };

    println!("cargo:warning=Using build script: {}", script);

    // Derleme sırasında uygun build script’i çalıştır
    let status = Command::new("rustc")
        .args(&[script, "--crate-type", "bin", "-o", "hdm_build_helper"])
        .status()
        .expect("Failed to compile helper build script");
    assert!(status.success(), "Failed to compile {}", script);

    let status = Command::new("./hdm_build_helper")
        .status()
        .expect("Failed to run helper build script");
    assert!(status.success(), "Helper build script failed");
}
