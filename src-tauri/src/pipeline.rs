use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PipelineStep {
    AdbCommand { args: Vec<String> },
    FastbootCommand { args: Vec<String> },
    Message { text: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlashPipeline {
    pub id: String,
    pub description: String,
    pub requires_adb: bool,
    pub requires_fastboot: bool,
    pub destructive: bool,
    pub steps: Vec<PipelineStep>,
}

pub fn list_builtin_pipelines() -> Vec<FlashPipeline> {
    vec![
        FlashPipeline {
            id: "reboot-chain".into(),
            description: "ADB → Bootloader → Fastboot verification".into(),
            requires_adb: true,
            requires_fastboot: true,
            destructive: false,
            steps: vec![
                PipelineStep::Message {
                    text: "Rebooting to bootloader".into(),
                },
                PipelineStep::AdbCommand {
                    args: vec!["reboot".into(), "bootloader".into()],
                },
                PipelineStep::Message {
                    text: "Waiting for fastboot".into(),
                },
            ],
        },
        FlashPipeline {
            id: "flash-boot-dry-run".into(),
            description: "Dry-run boot partition flash (no write)".into(),
            requires_adb: false,
            requires_fastboot: true,
            destructive: false,
            steps: vec![
                PipelineStep::FastbootCommand {
                    args: vec!["getvar".into(), "all".into()],
                },
            ],
        },
    ]
}
