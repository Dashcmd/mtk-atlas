use std::{thread, time::Duration};

use tauri::{AppHandle, Emitter, Manager};
use serde::Serialize;

use crate::{
    adb,
    adb::AdbState,
    app_state::AppState,
};

/// High-level device state exposed to the UI.
/// This is the ONLY state the frontend should depend on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DeviceState {
    Disconnected,
    AdbUnauthorized,
    AdbDevice,
    Fastboot,
    MtkPreloader,
}

/// Starts the background detection service.
/// Polls device state and emits events ONLY when state changes.
pub fn start_detection_service(app: AppHandle) {
    // IMPORTANT:
    // Clone the AppHandle so it can be moved into a 'static thread.
    let app_handle = app.clone();

    thread::spawn(move || {
        let state = app_handle.state::<AppState>();
        let mut last_state = DeviceState::Disconnected;

        loop {
            let new_state = detect_device_state();

            if new_state != last_state {
                // Update centralized AppState
                {
                    let mut locked = state.device_state.lock().unwrap();
                    *locked = new_state.clone();
                }

                // Notify frontend
                let _ = app_handle.emit("device_state_changed", new_state.clone());

                last_state = new_state;
            }

            thread::sleep(Duration::from_millis(800));
        }
    });
}

/// Determines the current device state.
/// ADB is authoritative for now.
fn detect_device_state() -> DeviceState {
    match adb::adb_state() {
        AdbState::Authorized => DeviceState::AdbDevice,
        AdbState::Unauthorized => DeviceState::AdbUnauthorized,
        _ => DeviceState::Disconnected,
    }
}
