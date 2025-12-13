use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MtkCapabilities {
    pub adb: bool,
    pub fastboot: bool,
    pub preloader: bool,
    pub brom: bool,

    pub description: String,
}

pub fn evaluate(
    adb: bool,
    fastboot: bool,
    mtk_state: &str,
) -> MtkCapabilities {
    let preloader = matches!(mtk_state, "MTK Preloader");
    let brom = preloader; // for now, treat preloader as BROM access

    let description = if brom {
        "BROM / Preloader access detected (dangerous)".to_string()
    } else if fastboot {
        "Fastboot mode available".to_string()
    } else if adb {
        "ADB access available".to_string()
    } else {
        "No active MediaTek interface".to_string()
    };

    MtkCapabilities {
        adb,
        fastboot,
        preloader,
        brom,
        description,
    }
}
