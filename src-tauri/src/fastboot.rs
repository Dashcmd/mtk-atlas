use std::process::Command;
use tauri::AppHandle;
use crate::logger::emit_log;

/// Execute a fastboot command.
/// ALWAYS considered dangerous.
pub fn fastboot_cmd(
    app: &AppHandle,
    args: &str,
) -> Result<String, String> {
    emit_log(app, "warn", format!("Fastboot → {}", args));

    let output = Command::new("fastboot")
        .args(args.split_whitespace())
        .output()
        .map_err(|e| {
            emit_log(app, "error", format!("Fastboot spawn failed: {}", e));
            e.to_string()
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        emit_log(app, "error", format!("Fastboot error: {}", err));
        Err(err)
    }
}
