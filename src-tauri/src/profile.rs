use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct DeviceProfile {
    pub device: DeviceInfo,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub model: Option<String>,
    pub soc: Option<String>,
    pub manufacturer: Option<String>,
}

pub fn load_profiles() -> Vec<DeviceProfile> {
    let mut profiles = Vec::new();

    if let Ok(entries) = fs::read_dir("devices") {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("yaml") {
                if let Ok(contents) = fs::read_to_string(entry.path()) {
                    if let Ok(profile) = serde_yaml::from_str::<DeviceProfile>(&contents) {
                        profiles.push(profile);
                    }
                }
            }
        }
    }

    profiles
}

pub fn match_profile(
    profiles: &[DeviceProfile],
    model: &str,
    soc: &str,
) -> String {
    for p in profiles {
        if let Some(m) = &p.device.model {
            if m == model {
                return p.device.name.clone();
            }
        }
    }

    for p in profiles {
        if let Some(s) = &p.device.soc {
            if s.eq_ignore_ascii_case(soc) {
                return p.device.name.clone();
            }
        }
    }

    "Generic MediaTek Device".to_string()
}
