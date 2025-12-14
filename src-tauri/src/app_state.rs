use std::sync::{Arc, Mutex};
use crate::detection_service::DeviceState;

pub struct AppState {
    pub device_state: Mutex<DeviceState>,
    pub root_state: Mutex<Option<crate::root::RootStatus>>,
    pub tools_installed: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            device_state: Mutex::new(DeviceState::Disconnected),
            root_state: Mutex::new(None),
            tools_installed: Mutex::new(false),
        })
    }
}
