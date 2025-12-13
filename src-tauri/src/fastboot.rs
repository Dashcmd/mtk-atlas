use std::process::Command;

/// Reboot device from fastboot mode (if applicable)
pub fn fastboot_reboot() -> Result<(), String> {
    let out = Command::new("fastboot")
        .arg("reboot")
        .output()
        .map_err(|e| e.to_string())?;

    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}

