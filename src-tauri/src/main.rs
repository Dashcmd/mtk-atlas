#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kernel;
mod profile;
mod adb;
mod fastboot;
mod pipeline;
mod executor;

#[tauri::command]
fn list_pipelines() -> Vec<pipeline::FlashPipeline> {
    pipeline::list_builtin_pipelines()
}

#[tauri::command]
fn run_pipeline(id: String, dry_run: bool) -> Result<(), String> {
    let pipelines = pipeline::list_builtin_pipelines();
    let pipeline = pipelines
        .into_iter()
        .find(|p| p.id == id)
        .ok_or("Pipeline not found")?;

    executor::execute_pipeline(&pipeline, dry_run)
}

#[tauri::command]
fn get_active_profile() -> String {
    let profiles = profile::load_profiles();
    profile::match_profile(&profiles, "", "")
}

#[tauri::command]
fn get_kernel_status() -> String {
    let details = kernel::detect_kernel_details();
    kernel::kernel_status_label(&details.status).to_string()
}

#[tauri::command]
fn get_kernel_capabilities() -> (bool, bool) {
    let details = kernel::detect_kernel_details();
    let caps = kernel::kernel_capabilities(&details);
    (caps.can_inspect, caps.can_modify)
}

#[tauri::command]
fn get_kernel_explanation() -> String {
    let details = kernel::detect_kernel_details();
    kernel::kernel_explanation(&details).to_string()
}

#[tauri::command]
fn get_adb_status() -> String {
    let state = adb::detect_adb_state();
    adb::adb_state_label(&state).to_string()
}

#[tauri::command]
fn get_adb_device_info() -> Option<(String, String)> {
    adb::adb_device_info()
}

#[tauri::command]
fn get_fastboot_status() -> bool {
    fastboot::fastboot_device_connected()
}

/* === ACTIONS === */

#[tauri::command]
fn adb_reboot_cmd() -> Result<(), String> {
    adb::adb_reboot()
}

#[tauri::command]
fn adb_reboot_recovery_cmd() -> Result<(), String> {
    adb::adb_reboot_recovery()
}

#[tauri::command]
fn adb_reboot_bootloader_cmd() -> Result<(), String> {
    adb::adb_reboot_bootloader()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
list_pipelines,
run_pipeline,
            get_active_profile,
            get_kernel_status,
            get_kernel_capabilities,
            get_kernel_explanation,
            get_adb_status,
            get_adb_device_info,
            get_fastboot_status,
            adb_reboot_cmd,
            adb_reboot_recovery_cmd,
            adb_reboot_bootloader_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
