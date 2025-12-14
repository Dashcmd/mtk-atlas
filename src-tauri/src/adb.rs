use std::process::Command;
use tauri::AppHandle;
use crate::logger::emit_log;

/// Execute a single adb shell command.
/// HARD-GATED by caller (ADB must be authorized).
pub fn adb_shell(
    app: &AppHandle,
    command: &str,
) -> Result<String, String> {
    emit_log(app, "info", format!("ADB shell → {}", command));

    let output = Command::new("adb")
        .args(["shell", command])
        .output()
        .map_err(|e| {
            emit_log(app, "error", format!("ADB spawn failed: {}", e));
            e.to_string()
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        emit_log(app, "error", format!("ADB error: {}", err));
        Err(err)
    }
}

/// One-shot device info query.
/// Returns (model, serial) if authorized.
pub fn adb_device_info(
    app: &AppHandle,
) -> Option<(String, String)> {
    let state = Command::new("adb")
        .args(["get-state"])
        .output()
        .ok()?;

    if String::from_utf8_lossy(&state.stdout).trim() != "device" {
        emit_log(app, "warn", "ADB not authorized");
        return None;
    }

    let model = Command::new("adb")
        .args(["shell", "getprop", "ro.product.model"])
        .output()
        .ok()?;

    let serial = Command::new("adb")
        .args(["get-serialno"])
        .output()
        .ok()?;

    emit_log(app, "info", "ADB device info queried");

    Some((
        String::from_utf8_lossy(&model.stdout).trim().to_string(),
        String::from_utf8_lossy(&serial.stdout).trim().to_string(),
    ))
}
