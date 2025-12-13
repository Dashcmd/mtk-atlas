use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::Command,
};

use tauri::State;

use crate::{
    app_state::AppState,
    detection_service::DeviceState,
};
fn classify_flash_risk(partition: &str) -> &'static str {
    match partition.to_lowercase().as_str() {
        "preloader" | "bootloader" | "lk" | "lk2" =>
            "CRITICAL: boot chain",

        "vbmeta" | "vbmeta_a" | "vbmeta_b" =>
            "CRITICAL: verified boot",

        "boot" | "boot_a" | "boot_b" |
        "vendor_boot" | "vendor_boot_a" | "vendor_boot_b" |
        "init_boot" | "init_boot_a" | "init_boot_b" =>
            "HIGH: kernel / ramdisk",

        "dtbo" | "dtbo_a" | "dtbo_b" =>
            "HIGH: device tree",

        "system" | "system_a" | "system_b" |
        "vendor" | "vendor_a" | "vendor_b" =>
            "MEDIUM: system image",

        _ =>
            "UNKNOWN / USER-SPECIFIED",
    }
}

#[tauri::command]
pub fn adb_run(
    state: State<AppState>,
    command: String,
) -> Result<String, String> {
    let device_state = state.device_state.lock().unwrap();

    if !matches!(*device_state, DeviceState::AdbDevice) {
        return Err("ADB not available (device not authorized)".into());
    }

    let out = Command::new("adb")
        .args(command.split_whitespace())
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

#[tauri::command]
pub fn fastboot_run(
    state: State<AppState>,
    expert: bool,
    command: String,
) -> Result<String, String> {
    if !expert {
        return Err("Expert mode required".into());
    }

    let device_state = state.device_state.lock().unwrap();

    if !matches!(*device_state, DeviceState::Fastboot) {
        return Err("Fastboot not active".into());
    }

    let out = Command::new("fastboot")
        .args(command.split_whitespace())
        .output()
        .map_err(|e| e.to_string())?;

    // fastboot writes to stderr by design
    Ok(String::from_utf8_lossy(&out.stderr).to_string())
}

#[tauri::command]
pub fn fastboot_flash(
    state: State<AppState>,
    partition: String,
    image: String,
) -> Result<String, String> {
    let device_state = state.device_state.lock().unwrap();

    // Only enforce reality: fastboot must be active
    if !matches!(*device_state, DeviceState::Fastboot) {
        return Err("Fastboot not active".into());
    }

    let out = Command::new("fastboot")
        .args(["flash", &partition, &image])
        .output()
        .map_err(|e| e.to_string())?;

    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}

#[tauri::command]
pub fn export_diagnostics(
    logs: String,
    device_state: String,
) -> Result<String, String> {
    use std::{
        fs::File,
        io::{Write, Cursor},
        path::PathBuf,
    };
    use zip::{ZipWriter, write::FileOptions};

    let mut path = PathBuf::from(std::env::temp_dir());
    path.push("mtk-atlas-diagnostics.zip");

    let file = File::create(&path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);

    let opts = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // ---- summary ----
    zip.start_file("summary.txt", opts)
        .map_err(|e| e.to_string())?;
    writeln!(
        zip,
        "MTK Atlas Diagnostics\nGenerated: {:?}",
        std::time::SystemTime::now()
    ).ok();

    // ---- device state ----
    zip.start_file("device_state.txt", opts)
        .map_err(|e| e.to_string())?;
    writeln!(zip, "Device State: {}", device_state).ok();

    // ---- logs ----
    zip.start_file("logs.txt", opts)
        .map_err(|e| e.to_string())?;
    zip.write_all(logs.as_bytes()).ok();

    zip.finish().map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().to_string())
}

