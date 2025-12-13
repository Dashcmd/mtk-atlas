use std::process::Command;

#[derive(Debug)]
pub enum AdbState {
    NoDevice,
    Unauthorized,
    Connected,
}

/* ===============================
   ADB STATE DETECTION
   =============================== */

pub fn detect_adb_state() -> AdbState {
    let output = Command::new("adb")
        .arg("devices")
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);

        if text.contains("\tdevice") {
            return AdbState::Connected;
        }

        if text.contains("\tunauthorized") {
            return AdbState::Unauthorized;
        }
    }

    AdbState::NoDevice
}

pub fn adb_state_label(state: &AdbState) -> &'static str {
    match state {
        AdbState::Connected => "Device connected",
        AdbState::Unauthorized => "Device unauthorized",
        AdbState::NoDevice => "No device",
    }
}

/* ===============================
   BOOLEAN CAPABILITY HELPER
   =============================== */

pub fn adb_connected() -> bool {
    matches!(detect_adb_state(), AdbState::Connected)
}

/* ===============================
   DEVICE INFO
   =============================== */

pub fn adb_device_info() -> Option<(String, String)> {
    if !adb_connected() {
        return None;
    }

    let model = Command::new("adb")
        .args(["shell", "getprop", "ro.product.model"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

    let serial = Command::new("adb")
        .args(["get-serialno"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

    Some((model, serial))
}

/* ===============================
   REBOOT ACTIONS (SAFE)
   =============================== */

pub fn adb_reboot() -> Result<(), String> {
    run_adb(&["reboot"])
}

pub fn adb_reboot_recovery() -> Result<(), String> {
    run_adb(&["reboot", "recovery"])
}

pub fn adb_reboot_bootloader() -> Result<(), String> {
    run_adb(&["reboot", "bootloader"])
}

/* ===============================
   INTERNAL HELPER
   =============================== */

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
