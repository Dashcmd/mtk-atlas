use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::Command,
};

use tauri::{AppHandle, State};

use crate::{
    app_state::AppState,
    detection_service::DeviceState,
    logger::emit_log,
    tools,
};

/* ================= PLATFORM TOOLS ================= */

#[tauri::command]
pub fn platform_tools_installed_cmd() -> bool {
    tools::platform_tools_installed()
}

#[tauri::command]
pub fn install_platform_tools_cmd(
    app: AppHandle,
    state: State<AppState>,
) -> Result<(), String> {
    tools::install_platform_tools(&app)?;
    *state.tools_installed.lock().unwrap() = true;
    Ok(())
}


/* ================= FLASH RISK ================= */

fn classify_flash_risk(partition: &str) -> &'static str {
    match partition.to_lowercase().as_str() {
        "preloader" | "bootloader" | "lk" | "lk2" =>
            "CRITICAL: boot chain",

        "vbmeta" | "vbmeta_a" | "vbmeta_b" =>
            "CRITICAL: verified boot",

        "boot" | "boot_a" | "boot_b"
        | "vendor_boot" | "vendor_boot_a" | "vendor_boot_b"
        | "init_boot" | "init_boot_a" | "init_boot_b" =>
            "HIGH: kernel / ramdisk",

        "dtbo" | "dtbo_a" | "dtbo_b" =>
            "HIGH: device tree",

        "system" | "system_a" | "system_b"
        | "vendor" | "vendor_a" | "vendor_b" =>
            "MEDIUM: system image",

        _ =>
            "UNKNOWN / USER-SPECIFIED",
    }
}

/* ================= ADB ================= */

#[tauri::command]
pub fn adb_run(
    app: AppHandle,
    state: State<AppState>,
    command: String,
) -> Result<String, String> {
    emit_log(&app, "info", format!("ADB run requested: {}", command));

    let device_state = state.device_state.lock().unwrap();

// Only block ADB if we are explicitly in fastboot or preloader
if matches!(
    *device_state,
    DeviceState::Fastboot | DeviceState::MtkPreloader
) {
    return Err("ADB not available in current device state".into());
}


    let mut parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty adb command".into());
    }

    // Allow "adb devices" or just "devices"
    if parts[0] == "adb" {
        parts.remove(0);
    }

    let adb = tools::adb_path();

    let out = Command::new(adb)
        .args(&parts)
        .output()
        .map_err(|e| {
            emit_log(&app, "error", format!("ADB spawn failed: {}", e));
            e.to_string()
        })?;

    let mut output = String::new();
    output.push_str(&String::from_utf8_lossy(&out.stdout));
    output.push_str(&String::from_utf8_lossy(&out.stderr));

    emit_log(&app, "info", "ADB command completed");

    Ok(output)
}


/* ================= FASTBOOT FLASH ================= */

#[tauri::command]
pub fn fastboot_run(
    app: AppHandle,
    state: State<AppState>,
    command: String,
) -> Result<String, String> {
    emit_log(
        &app,
        "warn",
        format!("Fastboot run requested: {}", command),
    );

    let device_state = state.device_state.lock().unwrap();

    if !matches!(*device_state, DeviceState::Fastboot) {
        return Err("Fastboot not active".into());
    }

    let mut parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty fastboot command".into());
    }

    // Allow "fastboot getvar all" or "getvar all"
    if parts[0] == "fastboot" {
        parts.remove(0);
    }

    let fastboot = tools::fastboot_path();

    let out = Command::new(fastboot)
        .args(&parts)
        .output()
        .map_err(|e| {
            emit_log(&app, "error", format!("Fastboot spawn failed: {}", e));
            e.to_string()
        })?;

    let mut output = String::new();
    output.push_str(&String::from_utf8_lossy(&out.stdout));
    output.push_str(&String::from_utf8_lossy(&out.stderr));

    emit_log(&app, "warn", "Fastboot command completed");

    Ok(output)
}

#[tauri::command]
pub fn fastboot_flash(
    app: AppHandle,
    state: State<AppState>,
    partition: String,
    image: String,
) -> Result<String, String> {
    emit_log(
        &app,
        "warn",
        format!("Fastboot flash requested: {} {}", partition, image),
    );

    let device_state = state.device_state.lock().unwrap();

    if !matches!(*device_state, DeviceState::Fastboot) {
        return Err("Fastboot not active".into());
    }

    let out = Command::new(tools::fastboot_path())
        .args(["flash", &partition, &image])
        .output()
        .map_err(|e| {
            emit_log(&app, "error", format!("Flash spawn failed: {}", e));
            e.to_string()
        })?;

    let mut output = String::new();
    output.push_str(&String::from_utf8_lossy(&out.stdout));
    output.push_str(&String::from_utf8_lossy(&out.stderr));

    if out.status.success() {
        emit_log(&app, "warn", format!("Flash SUCCESS → {}", partition));
        Ok(output)
    } else {
        emit_log(&app, "error", format!("Flash FAILED → {}", partition));
        Err(output)
    }
}




/* ================= DIAGNOSTICS ================= */

#[tauri::command]
pub fn export_diagnostics(
    app: AppHandle,
    logs: String,
    device_state: String,
) -> Result<String, String> {
    emit_log(&app, "info", "Diagnostics export requested");

    use zip::{ZipWriter, write::FileOptions};

    let mut path = PathBuf::from(std::env::temp_dir());
    path.push("mtk-atlas-diagnostics.zip");

    let file = File::create(&path).map_err(|e| {
        emit_log(&app, "error", format!("Zip create failed: {}", e));
        e.to_string()
    })?;

    let mut zip = ZipWriter::new(file);

    let opts = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    zip.start_file("summary.txt", opts).ok();
    writeln!(
        zip,
        "MTK Atlas Diagnostics\nGenerated: {:?}",
        std::time::SystemTime::now()
    ).ok();

    zip.start_file("device_state.txt", opts).ok();
    writeln!(zip, "Device State: {}", device_state).ok();

    zip.start_file("logs.txt", opts).ok();
    zip.write_all(logs.as_bytes()).ok();

    zip.finish().map_err(|e| {
        emit_log(&app, "error", format!("Zip finalize failed: {}", e));
        e.to_string()
    })?;

    emit_log(
        &app,
        "info",
        format!("Diagnostics written → {}", path.display()),
    );

    Ok(path.to_string_lossy().to_string())
}
