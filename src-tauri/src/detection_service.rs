use serde::Serialize;
use std::{
    sync::Arc,
    thread,
    time::Duration,
};

use tauri::{AppHandle, Emitter};

use crate::process::run;
use crate::app_state::AppState;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DeviceState {
    Disconnected,
    AdbUnauthorized,
    AdbDevice,
    Fastboot,
    MtkPreloader,
}

const POLL_INTERVAL_MS: u64 = 750;

pub fn start_detection_loop(app: AppHandle, state: Arc<AppState>) {
    thread::spawn(move || {
        let mut last_state = DeviceState::Disconnected;

        loop {
            let next_state = detect_state();

            if next_state != last_state {
                {
                    let mut guard = state.device_state.lock().unwrap();
                    *guard = next_state.clone();
                }

                let _ = app.emit("device-state", next_state.clone());
                last_state = next_state;
            }

            thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
        }
    });
}

fn detect_state() -> DeviceState {
    // 1) Fastboot
    if let Ok(out) = run("fastboot", &["devices"]) {
        if !String::from_utf8_lossy(&out.stdout).trim().is_empty() {
            return DeviceState::Fastboot;
        }
    }

    // 2) ADB
    if let Ok(out) = run("adb", &["get-state"]) {
        match String::from_utf8_lossy(&out.stdout).trim() {
            "device" => return DeviceState::AdbDevice,
            "unauthorized" => return DeviceState::AdbUnauthorized,
            _ => {}
        }
    }

    DeviceState::Disconnected
}
