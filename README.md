# MTK Atlas

**MTK Atlas** is a cross-platform desktop utility for detecting, managing, and interacting with Android devices — with a primary focus on **MediaTek (MTK)** platforms.

Built with **Tauri + Rust + SolidJS**, MTK Atlas prioritizes reliability, correctness, and low-level device state awareness over fragile scripting or opaque vendor tools.

---

## Features

### Device Detection
- Automatic USB state detection
- Live device mode tracking:
  - Disconnected
  - ADB (authorized / unauthorized)
  - Fastboot
  - MTK Preloader (analysis-only)
- Non-blocking, rate-limited detection loop

### Command Execution
- Raw **ADB** command runner
- Raw **Fastboot** command runner
- Device-state–gated execution (prevents invalid operations)
- Structured output and error reporting

### Logging & Diagnostics
- Live, timestamped logging panel
- Log levels (info / warning / error)
- Exportable diagnostics for troubleshooting

### Architecture
- Rust backend for hardware access and process execution
- SolidJS frontend for fast, reactive UI
- No background services
- No network communication
- Runs entirely locally

---

## Supported Platforms

- **Windows** (x64)
- **Linux** (Debian / Ubuntu)
- **macOS** (Intel & Apple Silicon)

---

## Requirements

- Android platform-tools (`adb`, `fastboot`)  
  MTK Atlas can detect and assist with installation if missing.

---

## Installation

Download the latest release from **GitHub Releases**:

👉 https://github.com/Dashcmd/mtk-atlas/releases

No installer required.

---

## Usage Notes

- ADB commands are only enabled when a device is authorized
- Fastboot commands are only enabled when the device is in Fastboot mode
- MTK Preloader detection is **read-only** (no flashing is performed)

---

## Disclaimer

MTK Atlas is a **diagnostic and development tool**.

Flashing firmware, unlocking bootloaders, or modifying partitions can permanently damage devices.  
You are solely responsible for how you use this software.
