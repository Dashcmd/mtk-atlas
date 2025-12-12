#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod profile;

#[tauri::command]
fn get_active_profile() -> String {
    // Phase 1: no adb yet
    // This ensures the app compiles and runs cleanly
    let profiles = profile::load_profiles();
    profile::match_profile(&profiles, "", "")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_active_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
