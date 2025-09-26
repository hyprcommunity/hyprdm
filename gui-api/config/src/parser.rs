use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

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

        if self.autologin && self.systemctl_usedefine {
            panic!("autologin and systemctl_usedefine cannot both be true!");
        }

        self
    }

    /// Kullanıcı tarafından seçilen arayüzün main.qml yolunu bulur
    pub fn find_quickshell_ui(&self) -> Option<PathBuf> {
        let interface_name = &self.default_session;

        let dirs = vec![
            dirs::config_dir().map(|d| d.join("hyprdm/quickshell")), // ~/.config/hyprdm/quickshell
            dirs::data_local_dir().map(|d| d.join("quickshell")),     // ~/.local/share/quickshell
            Some(PathBuf::from("/usr/share/hyprdm/quickshell")),      // /usr/share/hyprdm/quickshell
        ];

        for dir_opt in dirs {
            if let Some(dir) = dir_opt {
                let path = dir.join(interface_name).join("main.qml");
                if path.exists() && path.is_file() {
                    return Some(path);
                }
            }
        }

        None
    }
}

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

    if config.systemctl_usedefine {
        lines.push("systemctl_usedefine=true".to_string());
    } else {
        lines.push("# systemctl_usedefine=true".to_string());
    }

    fs::write(path, lines.join("\n")).map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(())
}

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
