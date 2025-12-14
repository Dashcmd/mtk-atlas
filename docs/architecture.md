\# MTK Atlas Architecture



MTK Atlas uses a local-only Tauri architecture.



\- SolidJS frontend

\- Rust backend

\- No network communication

\- All device access is gated by detected state



Device detection runs in a non-blocking loop and emits state changes to the UI.

TK Atlas grows.

