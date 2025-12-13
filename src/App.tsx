import { createResource, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [confirm, setConfirm] = createSignal<null | string>(null);

  const [kernelStatus] = createResource(() => invoke<string>("get_kernel_status"));
  const [kernelExplanation] = createResource(() => invoke<string>("get_kernel_explanation"));
  const [deviceProfile] = createResource(() => invoke<string>("get_active_profile"));
  const [kernelCaps] = createResource(() => invoke<[boolean, boolean]>("get_kernel_capabilities"));
  const [adbStatus] = createResource(() => invoke<string>("get_adb_status"));
  const [deviceInfo] = createResource(() => invoke<[string, string] | null>("get_adb_device_info"));
  const [fastbootConnected] = createResource(() => invoke<boolean>("get_fastboot_status"));

  const adbConnected = () => adbStatus() === "Device connected";
const [mtkState] = createResource(() =>
  invoke<string>("get_mtk_state")
);

  async function run(action: string, cmd: string) {
    setConfirm(null);
    await invoke(cmd);
  }

  return (
    <main class="container">
      <h1>MTK Atlas</h1>

      <div class="card">
        <div class="row"><span class="label">Profile</span><span>{deviceProfile()}</span></div>
        <div class="row"><span class="label">Kernel</span><span>{kernelStatus()}</span></div>
        <p class="label">{kernelExplanation()}</p>
      </div>

      <h2>Device</h2>
      <div class="card">
        {deviceInfo() ? (
          <>
            <div class="row"><span class="label">Model</span><span>{deviceInfo()![0]}</span></div>
            <div class="row"><span class="label">Serial</span><span>{deviceInfo()![1]}</span></div>
          </>
        ) : (
          <p class="label">No device info available</p>
        )}
      </div>

      <h2>ADB</h2>
      <div class="card">
        <div class="row"><span class="label">ADB</span><span>{adbStatus()}</span></div>
        <div class="row"><span class="label">Fastboot</span><span>{fastbootConnected() ? "Connected" : "Not detected"}</span></div>
      </div>

<h2>MediaTek Mode</h2>

<div class="card">
  <div class="row">
    <span class="label">MTK state</span>
    <span
      class={
        "status " +
        (mtkState() === "MTK Preloader detected"
          ? "warn"
          : mtkState() === "MTK BootROM detected"
          ? "bad"
          : "")
      }
    >
      {mtkState.loading ? "Detecting…" : mtkState()}
    </span>
  </div>

  <p class="label" style={{ "margin-top": "10px" }}>
    Preloader and BootROM modes are time-limited and device-restricted.
  </p>
</div>
      <h2>Actions</h2>
      <div class="card">
        <button class="action" disabled={!adbConnected()} onClick={() => setConfirm("reboot")}>Reboot</button>
        <button class="action" disabled={!adbConnected()} onClick={() => setConfirm("recovery")}>Recovery</button>
        <button class="action" disabled={!adbConnected()} onClick={() => setConfirm("bootloader")}>Bootloader</button>
        <button class="action" disabled>EDL (MTK)</button>
      </div>

      {confirm() && (
        <div class="card">
          <p class="label">Confirm {confirm()}?</p>
          <button class="action" onClick={() =>
            run(confirm()!, `adb_reboot_${confirm()}_cmd`)
          }>
            Yes, continue
          </button>
          <button class="action" onClick={() => setConfirm(null)}>Cancel</button>
        </div>
      )}
    </main>
  );
}

export default App;
