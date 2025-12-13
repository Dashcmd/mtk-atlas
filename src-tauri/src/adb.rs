use std::process::Command;

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

pub fn adb_device_info() -> Option<(String, String)> {
    let state = Command::new("adb").arg("get-state").output().ok()?;
    if String::from_utf8_lossy(&state.stdout).trim() != "device" {
        return None;
    }

    let model = Command::new("adb")
        .args(["shell", "getprop ro.product.model"])
        .output()
        .ok()?;

    let serial = Command::new("adb")
        .args(["get-serialno"])
        .output()
        .ok()?;

    Some((
        String::from_utf8_lossy(&model.stdout).trim().to_string(),
        String::from_utf8_lossy(&serial.stdout).trim().to_string(),
    ))
}
