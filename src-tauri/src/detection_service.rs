use std::{
    process::Command,
    thread,
    time::Duration,
};

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::logger::emit_log;

/* ================= DEVICE STATE ================= */

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum DeviceState {
    Disconnected,
    AdbUnauthorized,
    AdbDevice,
    Fastboot,
    MtkPreloader,
}

/* ================= ENTRY POINT ================= */

pub fn start_detection_loop(app: AppHandle) {
    thread::spawn(move || {
        emit_log(&app, "info", "Device detection loop started");

        let mut last_state = DeviceState::Disconnected;

        loop {
            let new_state = detect_state();

            if new_state != last_state {
                emit_log(
                    &app,
                    "info",
                    format!("Device state → {:?}", new_state),
                );

                let _ = app.emit("device_state_changed", new_state);
                last_state = new_state;
            }

            thread::sleep(Duration::from_millis(750));
        }
    });
}

/* ================= DETECTION ================= */

fn detect_state() -> DeviceState {
    // ---- Fastboot ----
    if let Ok(out) = Command::new("fastboot")
        .arg("devices")
        .output()
    {
        if !out.stdout.is_empty() {
            return DeviceState::Fastboot;
        }
    }

    // ---- ADB ----
    if let Ok(out) = Command::new("adb")
        .arg("get-state")
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout);

        if s.contains("device") {
            return DeviceState::AdbDevice;
        }

        if s.contains("unauthorized") {
            return DeviceState::AdbUnauthorized;
        }
    }

    // ---- MTK Preloader (stub for future) ----
    // This will later be replaced with USB VID/PID probing
    // for MediaTek preloader / BROM modes.
    //
    // return DeviceState::MtkPreloader;

    DeviceState::Disconnected
}
