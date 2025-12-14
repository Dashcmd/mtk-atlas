use std::process::{Command, Output};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn run(cmd: &str, args: &[&str]) -> Result<Output, String> {
    let mut c = Command::new(cmd);
    c.args(args);

    #[cfg(target_os = "windows")]
    c.creation_flags(CREATE_NO_WINDOW);

    c.output().map_err(|e| e.to_string())
}
