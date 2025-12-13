use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MtkCapabilities {
    pub adb: bool,
    pub fastboot: bool,
    pub preloader: bool,
    pub bootrom: bool,
}

pub fn evaluate(adb: bool, fastboot: bool, mtk_state: &str) -> MtkCapabilities {
    MtkCapabilities {
        adb,
        fastboot,
        preloader: mtk_state.contains("Preloader"),
        bootrom: mtk_state.contains("BootROM"),
    }
}
