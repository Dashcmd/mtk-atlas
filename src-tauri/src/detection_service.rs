use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    NoDevice,
    AdbUnauthorized,
    AdbDevice,
    FastbootDevice,
    MtkPreloader, // placeholder for later
}

pub fn start_detection_loop(state: Arc<Mutex<DeviceState>>) {
    thread::spawn(move || loop {
        let new_state = detect_device_state();
        let mut current = state.lock().unwrap();

        if *current != new_state {
            *current = new_state;
        }

        thread::sleep(Duration::from_millis(800));
    });
}

fn detect_device_state() -> DeviceState {
    // Fastboot has priority
    if let Ok(output) = Command::new("fastboot").arg("devices").output() {
        if !String::from_utf8_lossy(&output.stdout).trim().is_empty() {
            return DeviceState::FastbootDevice;
        }
    }

    // ADB detection
    if let Ok(output) = Command::new("adb").arg("get-state").output() {
        let state = String::from_utf8_lossy(&output.stdout).trim().to_string();

        return match state.as_str() {
            "device" => DeviceState::AdbDevice,
            "unauthorized" => DeviceState::AdbUnauthorized,
            _ => DeviceState::NoDevice,
        };
    }

    DeviceState::NoDevice
}
