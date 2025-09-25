use std::path::Path;
use std::process::Command;
use std::fs;
use std::env;
use std::time::UNIX_EPOCH;
use config::parser::load_or_create_config;

pub struct ConfigManager;

impl ConfigManager {
    pub fn run(reload: bool) {
        let config_path = Path::new("/etc/hyprdm/hyprdm.conf");

        if reload {
            // Reload kontrolü: değişip değişmediğine bak
            let last_modified = match fs::metadata(config_path)
                .and_then(|meta| meta.modified())
            {
                Ok(time) => time,
                Err(_) => {
                    eprintln!("Configuration file does not exist.");
                    return;
                }
            };

            let last_check_file = "/var/lib/hyprdm/config_last_check";
            let last_check = fs::read_to_string(last_check_file)
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);

            let last_modified_secs = last_modified
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if last_modified_secs <= last_check {
                eprintln!("Configuration not changed, nothing to reload.");
                return;
            }

            // Config değişmişse önce doğruluğunu kontrol et
            match load_or_create_config(config_path) {
                Ok(_) => {
                    // Config doğru, timestamp'i kaydet ve restart
                    if let Err(e) = fs::write(last_check_file, last_modified_secs.to_string()) {
                        eprintln!("Failed to update last check timestamp: {}", e);
                    }
                    println!("Configuration changed and valid, reloading...");
                    if let Err(e) = Command::new("sudo")
                        .arg("systemctl")
                        .arg("restart")
                        .arg("hyprdm")
                        .status()
                    {
                        eprintln!("Failed to restart hyprdm: {}", e);
                    } else {
                        println!("hyprdm restarted successfully.");
                    }
                }
                Err(_) => {
                    eprintln!("Config file corrupt. systemctl restart skipped.");
                    return;
                }
            }
        }

        // Ana config yönetim işlemleri
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
    if let Err(e) = fs::write(path, content) {
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
        if let Err(e) = fs::remove_file(path) {
            eprintln!("Failed to remove service file {}: {}", path, e);
        } else {
            println!("Service file {} removed", path);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reload = args.iter().any(|a| a == "--reload");
    ConfigManager::run(reload);
}
