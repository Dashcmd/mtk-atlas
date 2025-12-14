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
mod process;

use crate::{
    app_state::AppState,
    detection_service::start_detection_loop,
    logger::emit_log,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 1️⃣ Create shared state FIRST
    let app_state = AppState::new();

    tauri::Builder::default()
        // 2️⃣ Manage state before setup
        .manage(app_state.clone())
       .setup(move |app| {
    let app_handle = app.handle();

    emit_log(&app_handle, "info", "MTK Atlas starting");

    start_detection_loop(app_handle.clone(), app_state.clone());

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