use std::process::Command;

/// ADB connection / authorization state.
/// This is the ONLY authoritative signal for detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdbState {
    NoServer,
    NoDevice,
    Unauthorized,
    Authorized,
}

/// Execute a single adb shell command.
/// ON-DEMAND ONLY. Does NOT participate in detection.
pub fn adb_shell(command: &str) -> Result<String, String> {
    let output = Command::new("adb")
        .args(["shell", command])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// One-shot device info query.
/// HARD-GATED: only returns data if ADB is connected AND authorized.
pub fn adb_device_info() -> Option<(String, String)> {
    match adb_state() {
        AdbState::Authorized => {}
        _ => return None,
    }

    let model = Command::new("adb")
        .args(["shell", "getprop", "ro.product.model"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

    let serial = Command::new("adb")
        .arg("get-serialno")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

    if model.is_empty() || serial.is_empty() {
        None
    } else {
        Some((model, serial))
    }
}

/// Reboot device normally (ADB).
pub fn adb_reboot() -> Result<(), String> {
    run_adb(&["reboot"])
}

/// Reboot device into recovery (ADB).
pub fn adb_reboot_recovery() -> Result<(), String> {
    run_adb(&["reboot", "recovery"])
}

/// Reboot device into bootloader / fastboot (ADB).
pub fn adb_reboot_bootloader() -> Result<(), String> {
    run_adb(&["reboot", "bootloader"])
}

/// Internal helper for executing adb commands.
/// MUST NOT be used for detection.
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

/// Returns the current ADB authorization state.
/// Uses `adb devices` to correctly detect authorization.
pub fn adb_state() -> AdbState {
    let output = Command::new("adb")
        .args(["devices"])
        .output();

    let Ok(output) = output else {
        return AdbState::NoServer;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.ends_with("\tunauthorized") {
            return AdbState::Unauthorized;
        }
        if line.ends_with("\tdevice") {
            return AdbState::Authorized;
        }
    }

    AdbState::NoDevice
}
