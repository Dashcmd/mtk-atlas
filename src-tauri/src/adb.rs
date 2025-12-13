use std::process::Command;

#[derive(Debug)]
pub enum AdbState {
    NotInstalled,
    NoDevice,
    Unauthorized,
    Connected,
}

pub fn detect_adb_state() -> AdbState {
    let version = Command::new("adb").arg("version").output();
    if version.is_err() {
        return AdbState::NotInstalled;
    }

    let output = Command::new("adb").arg("devices").output();
    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);

        if text.contains("\tdevice") {
            AdbState::Connected
        } else if text.contains("\tunauthorized") {
            AdbState::Unauthorized
        } else {
            AdbState::NoDevice
        }
    } else {
        AdbState::NoDevice
    }
}

pub fn adb_state_label(state: &AdbState) -> &'static str {
    match state {
        AdbState::NotInstalled => "ADB not installed",
        AdbState::NoDevice => "No device connected",
        AdbState::Unauthorized => "Device unauthorized",
        AdbState::Connected => "Device connected",
    }
}

/* === DEVICE INFO === */

pub fn adb_device_info() -> Option<(String, String)> {
    let model = Command::new("adb")
        .args(["shell", "getprop", "ro.product.model"])
        .output()
        .ok()?;

    let serial = Command::new("adb")
        .arg("get-serialno")
        .output()
        .ok()?;

    Some((
        String::from_utf8_lossy(&model.stdout).trim().to_string(),
        String::from_utf8_lossy(&serial.stdout).trim().to_string(),
    ))
}

/* === REAL ACTIONS === */

pub fn adb_reboot() -> Result<(), String> {
    run_adb(&["reboot"])
}

pub fn adb_reboot_recovery() -> Result<(), String> {
    run_adb(&["reboot", "recovery"])
}

pub fn adb_reboot_bootloader() -> Result<(), String> {
    run_adb(&["reboot", "bootloader"])
}

fn run_adb(args: &[&str]) -> Result<(), String> {
    let out = Command::new("adb")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}
