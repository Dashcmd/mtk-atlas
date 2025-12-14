#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod adb;
mod app_state;
mod detection_service;
mod fastboot;
mod logger;
mod root;
mod commands;


use crate::{
    app_state::AppState,
    detection_service::start_detection_loop,
    logger::emit_log,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            emit_log(&app_handle, "info", "MTK Atlas starting");

            // Start passive device detection loop
            start_detection_loop(app_handle.clone());

            Ok(())
        })
        .manage(AppState::new())
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
