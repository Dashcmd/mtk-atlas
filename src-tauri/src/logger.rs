use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct UiLog {
    pub level: &'static str,
    pub message: String,
}

pub fn emit_log(
    app: &AppHandle,
    level: &'static str,
    message: impl Into<String>,
) {
    let _ = app.emit(
        "ui_log",
        UiLog {
            level,
            message: message.into(),
        },
    );
}
