#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb;
mod app_state;
mod commands;
mod detection_service;
mod fastboot;
mod kernel;
mod logger;
mod mtk;
mod pipeline;
mod profile;
mod root;
mod tools;


use crate::{
    detection_service::start_detection_loop,
    logger::emit_log,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            emit_log(&app_handle, "info", "MTK Atlas starting");

            // Start passive device detection loop
            start_detection_loop(app_handle.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::adb_run,
            commands::fastboot_run,
            commands::fastboot_flash,
            commands::export_diagnostics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running MTK Atlas");
}
