use std::process::Command;

pub fn fastboot_present() -> bool {
    Command::new("fastboot").arg("--version").output().is_ok()
}

pub fn fastboot_device_connected() -> bool {
    let out = Command::new("fastboot").arg("devices").output();
    if let Ok(o) = out {
        !String::from_utf8_lossy(&o.stdout).trim().is_empty()
    } else {
        false
    }
}
