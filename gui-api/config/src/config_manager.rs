use std::path::Path;
use std::process::Command;
use crate::parser::load_or_create_config;

pub struct ConfigManager;

impl ConfigManager {
    pub fn run() {
        let config_path = Path::new("/etc/hyprdm/hyprdm.conf");

        let config = match load_or_create_config(config_path) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Failed to load or create config: {}", e);
                return;
            }
        };

        println!("Configuration loaded:");
        println!("Theme: {}", config.theme);
        println!("Tiling: {}", config.tiling);
        println!("Default session: {}", config.default_session);
        println!("Autologin: {}", config.autologin);
        println!("Two-factor enabled: {}", config.two_factor_enabled);
        if let Some(method) = &config.two_factor_method {
            println!("2FA method: {}", method);
        }
        if let Some(secret) = &config.two_factor_secret {
            println!("2FA secret: {}", secret);
        }
        println!("Systemctl usedefine: {}", config.systemctl_usedefine);

        let service_path = "/etc/systemd/system/hdm.service";

        match (config.autologin, config.systemctl_usedefine) {
            (true, false) => {
                println!("Autologin enabled, systemctl service disabled.");
                // Oturum başlatma kodu buraya eklenebilir
                remove_service_if_exists(service_path);
            }
            (false, true) => {
                println!("Systemctl service will be created and enabled.");
                create_service(service_path);
                enable_service("hdm.service");
            }
            (true, true) => {
                println!("Autologin and systemctl service both enabled.");
                create_service(service_path);
                enable_service("hdm.service");
                // Oturum başlatma kodu buraya eklenebilir
            }
            (false, false) => {
                println!("Neither autologin nor systemctl service enabled.");
                remove_service_if_exists(service_path);
            }
        }
    }
}

fn create_service(path: &str) {
    let content = r#"[Unit]
Description=HyprDM Display Manager
After=graphical.target

[Service]
Type=simple
ExecStart=/usr/bin/hyprdm
Restart=always

[Install]
WantedBy=multi-user.target
"#;
    if let Err(e) = std::fs::write(path, content) {
        eprintln!("Failed to write systemd service: {}", e);
    } else {
        println!("Service file written at {}", path);
    }
}

fn enable_service(name: &str) {
    match Command::new("systemctl").arg("enable").arg(name).status() {
        Ok(s) if s.success() => println!("{} enabled successfully", name),
        Ok(_) => eprintln!("Failed to enable {}", name),
        Err(e) => eprintln!("Error running systemctl enable {}: {}", name, e),
    }
}

fn remove_service_if_exists(path: &str) {
    if Path::new(path).exists() {
        if let Err(e) = std::fs::remove_file(path) {
            eprintln!("Failed to remove service file {}: {}", path, e);
        } else {
            println!("Service file {} removed", path);
        }
    }
}
