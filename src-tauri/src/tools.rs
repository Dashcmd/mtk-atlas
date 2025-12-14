use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use tauri::{AppHandle, Emitter};
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use zip::ZipArchive;

/* ================= CONSTANTS ================= */

const PLATFORM_TOOLS_URL: &str =
    "https://dl.google.com/android/repository/platform-tools-latest-windows.zip";

/*
  IMPORTANT:
  Replace this with the CURRENT official SHA-256 hash from Google
  when you are ready to lock the release.
*/
const PLATFORM_TOOLS_SHA256: &str =
    "12c2841f354e92a0eb2fd7bf6f0f9bf8538abce7bd6b060ac8349d6f6a61107c";

/* ================= EVENTS ================= */

#[derive(serde::Serialize, Clone)]
struct InstallProgress {
    stage: String,
    percent: u8,
}

/* ================= PATHS ================= */

pub fn tools_root_dir() -> PathBuf {
    let mut dir = dirs::data_local_dir().expect("No local data dir");
    dir.push("MTKAtlas");
    dir
}

pub fn platform_tools_dir() -> PathBuf {
    tools_root_dir().join("platform-tools")
}

pub fn adb_path() -> PathBuf {
    platform_tools_dir().join("adb.exe")
}

pub fn fastboot_path() -> PathBuf {
    platform_tools_dir().join("fastboot.exe")
}

pub fn platform_tools_installed() -> bool {
    adb_path().exists() && fastboot_path().exists()
}

/* ================= INSTALL ================= */

pub fn install_platform_tools(app: &AppHandle) -> Result<(), String> {
    let root = tools_root_dir();
    let zip_path = root.join("platform-tools.zip");

    fs::create_dir_all(&root).map_err(|e| e.to_string())?;

    /* ---- DOWNLOAD ---- */

    app.emit(
        "platform-tools-progress",
        InstallProgress { stage: "download".into(), percent: 10 },
    ).ok();

    let mut resp = get(PLATFORM_TOOLS_URL).map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Download failed: {}", resp.status()));
    }

    let mut zip_file = fs::File::create(&zip_path).map_err(|e| e.to_string())?;
    io::copy(&mut resp, &mut zip_file).map_err(|e| e.to_string())?;

    /* ---- CHECKSUM ---- */

    app.emit(
        "platform-tools-progress",
        InstallProgress { stage: "verify".into(), percent: 35 },
    ).ok();

    verify_checksum(&zip_path)?;

    /* ---- EXTRACT ---- */

    app.emit(
        "platform-tools-progress",
        InstallProgress { stage: "extract".into(), percent: 60 },
    ).ok();

    let file = fs::File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = root.join(entry.name());

        if entry.is_dir() {
            fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).ok();
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            io::copy(&mut entry, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    /* ---- FINALIZE ---- */

    fs::remove_file(&zip_path).ok();

    if !platform_tools_installed() {
        return Err("Platform-tools install completed but binaries missing".into());
    }

    app.emit(
        "platform-tools-progress",
        InstallProgress { stage: "complete".into(), percent: 100 },
    ).ok();

    Ok(())
}

/* ================= CHECKSUM ================= */

fn verify_checksum(path: &PathBuf) -> Result<(), String> {
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

    let hash = format!("{:x}", Sha256::digest(&buf));

    if hash != PLATFORM_TOOLS_SHA256 {
        return Err("Platform-tools checksum verification failed".into());
    }

    Ok(())
}
