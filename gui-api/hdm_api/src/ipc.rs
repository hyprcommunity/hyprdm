use std::process::Command;

pub struct HyprlandIPC;

impl HyprlandIPC {
    /// Sends a Hyprland IPC command
    pub fn send_command(&self, cmd: &str) -> Result<(), String> {
        let output = Command::new("hyprctl")
            .arg(cmd)
            .output()
            .map_err(|e| format!("IPC command failed: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "IPC command error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Returns the active window status
    pub fn get_status(&self) -> Result<String, String> {
        let output = Command::new("hyprctl")
            .arg("activewindow")
            .output()
            .map_err(|e| format!("Failed to get IPC status: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "IPC status error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Updates and returns the active window (alias for get_status)
    pub fn update_active_window(&self) -> Result<String, String> {
        self.get_status()
    }
}
