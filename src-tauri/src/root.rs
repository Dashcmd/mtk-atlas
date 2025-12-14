use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct RootStatus {
    pub has_su: bool,
}

pub fn detect_root_state() -> RootStatus {
    let output = Command::new("adb")
        .args(["shell", "which", "su"])
        .output();

    let has_su = match output {
        Ok(o) => !String::from_utf8_lossy(&o.stdout).trim().is_empty(),
        Err(_) => false,
    };

    RootStatus { has_su }
}