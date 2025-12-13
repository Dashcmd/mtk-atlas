use std::sync::Mutex;
use crate::detection_service::DeviceState;
use crate::root::RootStatus;

pub struct AppState {
    pub device_state: Mutex<DeviceState>,
    pub root_state: Mutex<Option<RootStatus>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {e
            device_state: Mutex::new(DeviceState::Disconnected),
            root_state: Mutex::new(None),
        }
    }
}
