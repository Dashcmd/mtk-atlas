#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod adb;
mod fastboot;
mod mtk;
mod detection_service;

use app_state::AppState;
use detection_service::DeviceState;
use tauri::State;

#[tauri::command]
fn get_adb_status(state: State<AppState>) -> String {
    match *state.device_state.lock().unwrap() {
        DeviceState::AdbDevice => "Device connected".into(),
        DeviceState::AdbUnauthorized => "Unauthorized".into(),
        _ => "Not connected".into(),
    }
}

#[tauri::command]
fn adb_shell(command: String) -> Result<String, String> {
    adb::adb_shell(&command)
}

#[tauri::command]
fn get_adb_device_info() -> Option<(String, String)> {
    adb::adb_device_info()
}

#[tauri::command]
fn get_fastboot_status(state: State<AppState>) -> bool {
    matches!(
        *state.device_state.lock().unwrap(),
        DeviceState::Fastboot
    )
}

#[tauri::command]
fn get_mtk_state(state: State<AppState>) -> String {
    match *state.device_state.lock().unwrap() {
        DeviceState::MtkPreloader => "MTK Preloader".into(),
        _ => "Idle".into(),
    }
}

#[tauri::command]
fn get_mtk_capabilities(
    state: State<AppState>,
) -> mtk::capabilities::MtkCapabilities {
    let device_state = state.device_state.lock().unwrap();

    let mtk_state_str = match *device_state {
        DeviceState::MtkPreloader => "MTK Preloader",
        _ => "Idle",
    };

    mtk::capabilities::evaluate(
        matches!(*device_state, DeviceState::AdbDevice),
        matches!(*device_state, DeviceState::Fastboot),
        mtk_state_str,
    )
}

#[tauri::command]
fn adb_reboot() -> Result<(), String> {
    adb::adb_reboot()
}

#[tauri::command]
fn adb_reboot_recovery() -> Result<(), String> {
    adb::adb_reboot_recovery()
}

#[tauri::command]
fn adb_reboot_bootloader() -> Result<(), String> {
    adb::adb_reboot_bootloader()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            device_state: std::sync::Mutex::new(DeviceState::Disconnected),
        })
        .setup(|app| {
            detection_service::start_detection_service(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_adb_status,
            get_adb_device_info,
            get_fastboot_status,
            get_mtk_state,
            get_mtk_capabilities,
            adb_reboot,
            adb_reboot_recovery,
            adb_reboot_bootloader,
            adb_shell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
