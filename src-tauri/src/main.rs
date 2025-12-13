#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kernel;
mod profile;

#[tauri::command]
fn get_kernel_capabilities() -> (bool, bool) {
    let details = kernel::detect_kernel_details();
    let caps = kernel::kernel_capabilities(&details);
    (caps.can_inspect, caps.can_modify)
}

#[tauri::command]
fn get_active_profile() -> String {
    let profiles = profile::load_profiles();
    profile::match_profile(&profiles, "", "")
}

#[tauri::command]
fn get_kernel_explanation() -> String {
    let details = kernel::detect_kernel_details();
    kernel::kernel_explanation(&details).to_string()
}

#[tauri::command]
fn get_kernel_status() -> String {
    let details = kernel::detect_kernel_details();
    kernel::kernel_status_label(&details.status).to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_active_profile,
            get_kernel_status,
            get_kernel_capabilities,
get_kernel_explanation
	 ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
