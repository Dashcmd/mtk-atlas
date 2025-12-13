use std::process::Command;
use crate::pipeline::{FlashPipeline, PipelineStep};

pub fn execute_pipeline(pipeline: &FlashPipeline, dry_run: bool) -> Result<(), String> {
    for step in &pipeline.steps {
        match step {
            PipelineStep::Message { text } => {
                println!("[PIPELINE] {}", text);
            }

            PipelineStep::AdbCommand { args } => {
                if dry_run {
                    println!("[DRY-RUN] adb {:?}", args);
                    continue;
                }

                let status = Command::new("adb")
                    .args(args)
                    .status()
                    .map_err(|e| e.to_string())?;

                if !status.success() {
                    return Err("ADB command failed".into());
                }
            }

            PipelineStep::FastbootCommand { args } => {
                if dry_run {
                    println!("[DRY-RUN] fastboot {:?}", args);
                    continue;
                }

                let status = Command::new("fastboot")
                    .args(args)
                    .status()
                    .map_err(|e| e.to_string())?;

                if !status.success() {
                    return Err("Fastboot command failed".into());
                }
            }
        }
    }

    Ok(())
}
