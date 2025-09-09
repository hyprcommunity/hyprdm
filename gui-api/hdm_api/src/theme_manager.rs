use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use config::{HDMConfig, save_config};
use crate::ipc::HyprlandIPC;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ThemeType {
    Gtk3,
    Gtk4,
    Qt5,
    Qt6,
    HyprSensivityObjective,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ThemeTarget {
    pub platform: ThemeType,
    pub dir_path: PathBuf,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UnidataScrub {
    pub targets: Vec<ThemeTarget>,
}

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
    pub kind: ThemeType,
}

pub struct ThemeManager {
    pub themes: Vec<Theme>,
    pub current: Option<Theme>,
    pub config: HDMConfig,
    pub config_path: PathBuf,
}

impl ThemeManager {
    pub fn new(config: HDMConfig, config_path: PathBuf) -> Self {
        Self { themes: vec![], current: None, config, config_path }
    }

    pub fn load_and_apply_themes(&mut self) -> Result<(), String> {
        let home = std::env::var("HOME").map_err(|_| "HOME env not found".to_string())?;
        let scrub_path = Path::new(&home)
            .join(".local/share/hyprdm/unidata/unidata.scrub");

        if !scrub_path.exists() {
            return Err(format!("unidata.scrub not found: {:?}", scrub_path));
        }

        let content = fs::read_to_string(&scrub_path).map_err(|e| e.to_string())?;
        let scrub: UnidataScrub = serde_json::from_str(&content).map_err(|e| e.to_string())?;

        for target in scrub.targets {
            self.scan_dir_for_themes(target.dir_path, target.platform.clone())?;
        }

        if let Some(theme) = self.themes.first() {
            self.apply_theme(theme)?;
            self.current = Some(theme.clone());
            self.config.theme = theme.name.clone();
            save_config(&self.config_path, &self.config)?;
        }

        Ok(())
    }

    fn scan_dir_for_themes(&mut self, path: PathBuf, kind: ThemeType) -> Result<(), String> {
        if path.exists() && path.is_dir() {
            for entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let p = entry.path();
                if p.is_dir() {
                    let name = p.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    self.themes.push(Theme { name, path: p, kind: kind.clone() });
                }
            }
        }
        Ok(())
    }

    pub fn available_themes_for_platform(&self, platform: ThemeType) -> Vec<String> {
        self.themes.iter()
            .filter(|t| t.kind == platform)
            .map(|t| t.name.clone())
            .collect()
    }

    pub fn set_theme(&mut self, name: &str) -> Result<(), String> {
        if let Some(theme) = self.themes.iter().find(|t| t.name == name) {
            self.apply_theme(theme)?;
            self.current = Some(theme.clone());
            self.config.theme = theme.name.clone();
            save_config(&self.config_path, &self.config)?;
            Ok(())
        } else {
            Err(format!("Theme not found: {}", name))
        }
    }

    fn apply_theme(&self, theme: &Theme) -> Result<(), String> {
        match theme.kind {
            ThemeType::Gtk3 => self.apply_gtk_theme(&theme.name, 3)?,
            ThemeType::Gtk4 => self.apply_gtk_theme(&theme.name, 4)?,
            ThemeType::Qt5 | ThemeType::Qt6 => self.apply_qt_theme(theme)?,
            ThemeType::HyprSensivityObjective => self.apply_hypersensivity_theme(&theme.path)?,
        }
        Ok(())
    }

    fn apply_gtk_theme(&self, theme_name: &str, version: u8) -> Result<(), String> {
        let home = std::env::var("HOME").map_err(|_| "HOME env not found".to_string())?;
        let config_path = match version {
            3 => format!("{}/.config/gtk-3.0/settings.ini", home),
            4 => format!("{}/.config/gtk-4.0/settings.ini", home),
            _ => return Err("Unknown GTK version".into()),
        };
        let content = format!("[Settings]\ngtk-theme-name={}\n", theme_name);
        fs::create_dir_all(Path::new(&config_path).parent().unwrap())
            .map_err(|e| e.to_string())?;
        fs::write(&config_path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn apply_qt_theme(&self, theme: &Theme) -> Result<(), String> {
        let qss_files = vec![
            theme.path.join("style.qss"),
            theme.path.join("theme.qss"),
        ];

        for qss in qss_files {
            if qss.exists() {
                std::env::set_var("QT_STYLE_OVERRIDE", &theme.name);
                break;
            }
        }
        Ok(())
    }

    fn apply_hypersensivity_theme(&self, theme_path: &Path) -> Result<(), String> {
        let config_file = theme_path.join("theme.json");
        if !config_file.exists() {
            return Err("HyprSensivity theme JSON not found".into());
        }

        let content = fs::read_to_string(&config_file).map_err(|e| e.to_string())?;
        let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

        if let Some(bg) = json.get("background") {
            let bg_str = bg.as_str().ok_or("Background value is not a string")?;
            let ipc = HyprlandIPC;
            ipc.send_command(&format!("setv window:background {}", bg_str))?;
        }

        Ok(())
    }
} 
