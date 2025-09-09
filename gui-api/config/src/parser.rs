use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HDMConfig {
    pub theme: String,
    pub tiling: bool,
    pub default_session: String,
    pub autologin: bool,
    pub two_factor_enabled: bool,
    pub two_factor_method: Option<String>,
    pub two_factor_secret: Option<String>,
    pub systemctl_usedefine: bool,
}

impl HDMConfig {
    fn validate(mut self) -> Self {
        if self.theme.is_empty() {
            self.theme = "Adwaita-dark".to_string();
        }
        if self.default_session.is_empty() {
            self.default_session = "Hyprland".to_string();
        }

        // autologin and systemctl_usedefine cannot both be true
        if self.autologin && self.systemctl_usedefine {
            panic!("autologin and systemctl_usedefine cannot both be true!");
        }

        self
    }
}

/// Load and validate the config
pub fn load_config(path: &Path) -> Result<HDMConfig, String> {
    if !path.exists() {
        return Ok(HDMConfig {
            theme: "Adwaita-dark".into(),
            tiling: true,
            default_session: "Hyprland".into(),
            autologin: true,
            two_factor_enabled: false,
            two_factor_method: None,
            two_factor_secret: None,
            systemctl_usedefine: false,
        });
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let config = HDMConfig {
        theme: map.get("theme").cloned().unwrap_or_else(|| "Adwaita-dark".into()),
        tiling: map.get("tiling").map(|v| v == "true").unwrap_or(true),
        default_session: map.get("default_session").cloned().unwrap_or_else(|| "Hyprland".into()),
        autologin: map.get("autologin").map(|v| v == "true").unwrap_or(true),
        two_factor_enabled: map.get("two_factor_enabled").map(|v| v == "true").unwrap_or(false),
        two_factor_method: map.get("two_factor_method").cloned(),
        two_factor_secret: map.get("two_factor_secret").cloned(),
        systemctl_usedefine: map.get("systemctl_usedefine").map(|v| v == "true").unwrap_or(false),
    };

    Ok(config.validate())
}

/// Save the config to disk
pub fn save_config(path: &Path, config: &HDMConfig) -> Result<(), String> {
    let mut lines = vec![];
    lines.push(format!("theme={}", config.theme));
    lines.push(format!("tiling={}", config.tiling));
    lines.push(format!("default_session={}", config.default_session));
    lines.push(format!("autologin={}", config.autologin));
    lines.push(format!("two_factor_enabled={}", config.two_factor_enabled));
    if let Some(method) = &config.two_factor_method {
        lines.push(format!("two_factor_method={}", method));
    }
    if let Some(secret) = &config.two_factor_secret {
        lines.push(format!("two_factor_secret={}", secret));
    }

    // Add systemctl_usedefine as a commented or active line
    if config.systemctl_usedefine {
        lines.push("systemctl_usedefine=true".to_string());
    } else {
        lines.push("# systemctl_usedefine=true".to_string());
    }

    let content = lines.join("\n");
    fs::write(path, content).map_err(|e| format!("Failed to save config: {}", e))?;

    // If systemctl flag is true, create and enable the service automatically
    if config.systemctl_usedefine {
        create_and_enable_service()?;
    }

    Ok(())
}

/// Create a systemd service under multi-user.target and enable it
fn create_and_enable_service() -> Result<(), String> {
    let service_path = "/etc/systemd/system/multi-user.target.wants/hdm.service";
    let service_content = r#"[Unit]
Description=HyprDM Display Manager
After=graphical.target

[Service]
Type=simple
ExecStart=/usr/bin/hyprdm
Restart=always

[Install]
WantedBy=multi-user.target
"#;

    fs::write(service_path, service_content)
        .map_err(|e| format!("Failed to create systemd service: {}", e))?;

    // Enable service via systemctl
    let status = Command::new("systemctl")
        .arg("enable")
        .arg("hdm.service")
        .status()
        .map_err(|e| format!("Failed to enable service: {}", e))?;

    if !status.success() {
        return Err("systemctl enable failed".into());
    }

    Ok(())
}

/// Load config or create default if not exists
pub fn load_or_create_config(path: &Path) -> Result<HDMConfig, String> {
    if path.exists() {
        load_config(path)
    } else {
        let default = HDMConfig {
            theme: "Adwaita-dark".into(),
            tiling: true,
            default_session: "Hyprland".into(),
            autologin: true,
            two_factor_enabled: false,
            two_factor_method: None,
            two_factor_secret: None,
            systemctl_usedefine: false,
        };
        save_config(path, &default)?;
        Ok(default)
    }
}
