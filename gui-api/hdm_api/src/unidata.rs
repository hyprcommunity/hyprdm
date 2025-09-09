use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum PlatformType {
    Gtk3,
    Gtk4,
    Qt5,
    Qt6,
    HyprSensivityObjective,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeTarget {
    pub platform: PlatformType,
    pub dir_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct UnidataScrub {
    pub targets: Vec<ThemeTarget>,
}

pub struct UnidataGenerator {
    pub scrub_path: PathBuf,
    pub data: UnidataScrub,
}

impl UnidataGenerator {
    pub fn new(scrub_path: &str) -> Self {
        Self {
            scrub_path: PathBuf::from(scrub_path),
            data: UnidataScrub { targets: vec![] },
        }
    }

    /// Dinamik olarak dizin ekle
    pub fn add_target_dir(&mut self, platform: PlatformType, dir_path: String) {
        self.data.targets.push(ThemeTarget {
            platform,
            dir_path: PathBuf::from(dir_path),
        });
    }

    /// Sistemden veya env üzerinden theme dizinlerini al
    pub fn add_system_theme_dirs(&mut self) {
        // XDG_DATA_DIRS
        if let Ok(xdg_dirs) = std::env::var("XDG_DATA_DIRS") {
            for dir in xdg_dirs.split(':') {
                let gtk3 = format!("{}/themes", dir);
                let gtk4 = format!("{}/themes", dir);
                self.add_target_dir(PlatformType::Gtk3, gtk3);
                self.add_target_dir(PlatformType::Gtk4, gtk4);
            }
        }

        // HOME dizini
        if let Ok(home) = std::env::var("HOME") {
            let gtk3_local = format!("{}/.themes", home);
            let gtk4_local = format!("{}/.local/share/themes", home);
            self.add_target_dir(PlatformType::Gtk3, gtk3_local);
            self.add_target_dir(PlatformType::Gtk4, gtk4_local);
        }

        // Qt5/Qt6 env değişkeni
        if let Ok(qt5) = std::env::var("QT5_THEME_DIR") {
            self.add_target_dir(PlatformType::Qt5, qt5);
        }
        if let Ok(qt6) = std::env::var("QT6_THEME_DIR") {
            self.add_target_dir(PlatformType::Qt6, qt6);
        }

        // HyprSensivity-Objective env değişkeni
        if let Ok(hyp) = std::env::var("HYPERSENSIVITY_THEME_DIR") {
            self.add_target_dir(PlatformType::HyprSensivityObjective, hyp);
        }
    }

    /// unidata.scrub dosyasını oluştur
    pub fn write_scrub(&self) -> Result<(), String> {
        if let Some(parent) = self.scrub_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut file = File::create(&self.scrub_path).map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&self.data).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
