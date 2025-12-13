use std::process::Command;

pub fn fastboot_cmd(command: &str) -> Result<String, String> {
    let args: Vec<&str> = command.split_whitespace().collect();

    let output = Command::new("fastboot")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
