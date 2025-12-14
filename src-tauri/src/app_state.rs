use std::sync::Mutex;

use crate::detection_service::DeviceState;
use crate::root::RootStatus;
use crate::tools;

pub struct AppState {
    pub device_state: Mutex<DeviceState>,
    pub root_state: Mutex<Option<RootStatus>>,
    pub tools_installed: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            device_state: Mutex::new(DeviceState::Disconnected),
            root_state: Mutex::new(None),
            tools_installed: Mutex::new(tools::platform_tools_installed()),
        }
    }
}