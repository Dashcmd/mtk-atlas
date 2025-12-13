#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb;
mod app_state;
mod detection_service;
mod fastboot;
mod logger;
mod root;

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

use app_state::AppState;
use detection_service::{DeviceState, start_detection_loop};

#[tauri::command]
fn get_device_state(state: State<AppState>) -> DeviceState {
    state.device_state.lock().unwrap().clone()
}

#[tauri::command]
fn get_root_state(state: State<AppState>) -> Option<root::RootStatus> {
    state.root_state.lock().unwrap().clone()
}

#[tauri::command]
fn adb_shell_cmd(cmd: String) -> Result<String, String> {
    adb::adb_shell(&cmd)
}

#[tauri::command]
fn fastboot_cmd(cmd: String) -> Result<String, String> {
    fastboot::fastboot_cmd(&cmd)
}

fn main() {
    let app_state = AppState::new();

    let detection_state = Arc::new(Mutex::new(DeviceState::NoDevice));
    start_detection_loop(detection_state.clone());

    tauri::Builder::default()
        .manage(app_state)
        .setup(move |app| {
            let handle = app.handle();
            let state_ref = handle.state::<AppState>();

            // Sync detection loop → AppState
            std::thread::spawn(move || loop {
                let detected = detection_state.lock().unwrap().clone();
                let mut stored = state_ref.device_state.lock().unwrap();
                *stored = detected;
                std::thread::sleep(std::time::Duration::from_millis(500));
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_device_state,
            get_root_state,
            adb_shell_cmd,
            fastboot_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error running MTK Atlas");
}
