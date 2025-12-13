use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum KernelStatus {
    Usable,
    Limited,
    Unsupported,
}

#[derive(Debug)]
pub struct KernelDetails {
    pub status: KernelStatus,
    pub config_present: bool,
    pub config_readable: bool,
}

#[derive(Debug)]
pub struct KernelCapabilities {
    pub can_inspect: bool,
    pub can_modify: bool,
}

pub fn detect_kernel_details() -> KernelDetails {
    let config_path = Path::new("/proc/config.gz");

    let present = config_path.exists();
    let readable = present && fs::read(config_path).is_ok();

    let status = if readable {
        KernelStatus::Usable
    } else if present {
        KernelStatus::Limited
    } else {
        KernelStatus::Unsupported
    };

    KernelDetails {
        status,
        config_present: present,
        config_readable: readable,
    }
}

pub fn kernel_capabilities(details: &KernelDetails) -> KernelCapabilities {
    KernelCapabilities {
        can_inspect: details.config_readable,
        // Modification requires root / KernelSU / Magisk (future)
        can_modify: false,
    }
}

pub fn kernel_status_label(status: &KernelStatus) -> &'static str {
    match status {
        KernelStatus::Usable => "Usable",
        KernelStatus::Limited => "Limited",
        KernelStatus::Unsupported => "Unsupported",
    }
}

pub fn kernel_explanation(details: &KernelDetails) -> &'static str {
    match details.status {
        KernelStatus::Usable => {
            "Kernel configuration is accessible and can be inspected."
        }
        KernelStatus::Limited => {
            "Kernel configuration exists but is not readable without elevated permissions."
        }
        KernelStatus::Unsupported => {
            "No Linux kernel environment detected on this system."
        }
    }
}

