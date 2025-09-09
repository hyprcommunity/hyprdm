use std::process::{Command, Child};
use std::fs;
use std::path::Path;

/// Represents a Wayland session
#[derive(Debug)]
pub struct Session {
    pub name: String,
    pub exec: String,
    pub child: Option<Child>,
}

impl Session {
    /// Create a new session
    pub fn new(name: &str, exec: &str) -> Self {
        Self {
            name: name.to_string(),
            exec: exec.to_string(),
            child: None,
        }
    }

    /// Start the session safely
    pub fn start(&mut self) -> Result<(), String> {
        if self.child.is_some() {
            return Err("Session is already running".into());
        }

        let child = Command::new(&self.exec)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| format!("Session start error: {}", e))?;

        self.child = Some(child);
        Ok(())
    }

    /// Stop the session safely
    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.child.take() {
            // Kill the process and wait for it to exit
            child.kill().map_err(|e| format!("Session stop error: {}", e))?;
            child.wait().map_err(|e| format!("Failed to wait for child: {}", e))?;
        }
        Ok(())
    }

    /// Restart the session
    pub fn restart(&mut self) -> Result<(), String> {
        self.stop()?;
        self.start()?;
        Ok(())
    }

    /// Switch to a new session
    pub fn switch_session(&mut self, new_exec: &str, new_name: &str) -> Result<(), String> {
        self.stop()?;
        self.exec = new_exec.to_string();
        self.name = new_name.to_string();
        self.start()?;
        Ok(())
    }

    /// Read all available Wayland session desktop files from the system
    pub fn available_sessions() -> Vec<Session> {
        let mut sessions = Vec::new();
        let paths = vec![
            "/usr/share/wayland-sessions",
            "/usr/share/wayland-session",
        ];

        for path in paths {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let mut name = String::new();
                            let mut exec = String::new();
                            let mut hidden = false;

                            for line in content.lines() {
                                let line = line.trim();
                                if line.starts_with("Name=") {
                                    name = line["Name=".len()..].to_string();
                                } else if line.starts_with("Exec=") {
                                    exec = line["Exec=".len()..].to_string();
                                } else if line.starts_with("Hidden=") && line.ends_with("true") {
                                    hidden = true;
                                } else if line.starts_with("NoDisplay=") && line.ends_with("true") {
                                    hidden = true;
                                }
                            }

                            if !name.is_empty() && !exec.is_empty() && !hidden {
                                // Clean up %U, %u, %F placeholders
                                exec = exec.split_whitespace().next().unwrap_or("").to_string();
                                sessions.push(Session::new(&name, &exec));
                            }
                        }
                    }
                }
            }
        }

        sessions
    }
}
