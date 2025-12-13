use std::process::Command;

#[derive(Debug, Clone)]
pub enum MtkState {
    Preloader,
    Brom,
    NotDetected,
}

pub fn detect_mtk_state() -> MtkState {
    if cfg!(target_os = "windows") {
        detect_windows()
    } else {
        detect_unix()
    }
}

/* ================= WINDOWS ================= */

fn detect_windows() -> MtkState {
    let output = Command::new("powershell")
        .args([
            "-Command",
            "Get-PnpDevice | Where-Object { $_.InstanceId -match 'VID_0E8D' }",
        ])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout).to_lowercase();

        if text.contains("preloader") {
            return MtkState::Preloader;
        }

        if text.contains("mediatek") {
            return MtkState::Brom;
        }
    }

    MtkState::NotDetected
}

/* ================= LINUX / MAC ================= */

fn detect_unix() -> MtkState {
    let output = Command::new("lsusb").output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout).to_lowercase();

        if text.contains("0e8d") {
            return MtkState::Preloader;
        }
    }

    MtkState::NotDetected
}

pub fn mtk_state_label(state: &MtkState) -> &'static str {
    match state {
        MtkState::Preloader => "MTK Preloader detected",
        MtkState::Brom => "MTK BootROM detected",
        MtkState::NotDetected => "No MTK device detected",
    }
}
