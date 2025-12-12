\# MTK Atlas Architecture



MTK Atlas is a cross-platform desktop application built with:



\- Tauri (Rust backend)

\- SolidJS (TypeScript frontend)

\- External Android tools (ADB / Fastboot)



\## Design Goals



\- Safety-first behavior

\- Device profile–based feature gating

\- Kernel capability detection

\- No proprietary binaries

\- No hidden execution



\## High-Level Structure



\- Frontend: SolidJS UI panels

\- Backend: Rust services exposed via Tauri commands

\- Data: Device profiles and scripts stored as readable files



This document will evolve as MTK Atlas grows.



