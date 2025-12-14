#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod adb;
mod app_state;
mod detection_service;
mod fastboot;
mod logger;
mod root;
mod commands;
mod process;

use crate::{
    app_state::AppState,
    detection_service::start_detection_loop,
    logger::emit_log,
};

fn main() {
    // 1️⃣ Create shared state FIRST
    let app_state = AppState::new();

    tauri::Builder::default()
        // 2️⃣ Manage it before setup
        .manage(app_state.clone())
        .setup(move |app| {
            let app_handle = app.handle();

            emit_log(&app_handle, "info", "MTK Atlas starting");

            // 3️⃣ Start detection ONCE with state
            start_detection_loop(app_handle.clone(), app_state.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::adb_run,
            commands::fastboot_run,
            commands::fastboot_flash,
            commands::export_diagnostics,
            commands::platform_tools_installed_cmd,
            commands::install_platform_tools_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error running MTK Atlas");
}
