use std::sync::Mutex;
use crate::detection_service::DeviceState;

pub struct AppState {
    pub device_state: Mutex<DeviceState>,
}
