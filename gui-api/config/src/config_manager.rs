use std::path::Path;
use std::process::Command;
use crate::parser::{HDMConfig, load_or_create_config};

pub struct ConfigManager;

impl ConfigManager {
    pub fn run() {
        let config_path = Path::new("/etc/hyprdm/hyprdm.conf");

        // Load existing config or create default
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

        // Handle autologin or systemctl
        if config.autologin {
            println!("Autologin is enabled. User session will start automatically.");
        } else if config.systemctl_usedefine {
            println!("Systemctl service will be enabled.");
            let status = Command::new("systemctl")
                .arg("status")
                .arg("hdm.service")
                .status();

            match status {
                Ok(s) if s.success() => println!("hdm.service is active"),
                Ok(_) => println!("hdm.service exists but is not active"),
                Err(e) => eprintln!("Failed to check systemctl status: {}", e),
            }
        } else {
            println!("Neither autologin nor systemctl service is enabled.");
        }
    }
}
