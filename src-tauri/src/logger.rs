use tauri::AppHandle;

pub fn emit_log(app: &AppHandle, message: &str) {
    let _ = app.emit_all("log", message.to_string());
}
