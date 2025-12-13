import { createResource } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [kernelStatus] = createResource(() =>
    invoke<string>("get_kernel_status")
  );

  const [kernelExplanation] = createResource(() =>
    invoke<string>("get_kernel_explanation")
  );

  const [deviceProfile] = createResource(() =>
    invoke<string>("get_active_profile")
  );

  const [kernelCaps] = createResource(() =>
    invoke<[boolean, boolean]>("get_kernel_capabilities")
  );

  const [adbStatus] = createResource(() =>
    invoke<string>("get_adb_status")
  );

  const adbConnected = () => adbStatus() === "Device connected";

  return (
    <main class="container">
      <h1>MTK Atlas</h1>

      {/* Device + Kernel */}
      <div class="card">
        <div class="row">
          <span class="label">Active Device Profile</span>
          <span class="status">
            {deviceProfile.loading ? "Detecting…" : deviceProfile()}
          </span>
        </div>

        <div class="row">
          <span class="label">Kernel Status</span>
          <span
            class={
              "status " +
              (kernelStatus() === "Usable"
                ? "good"
                : kernelStatus() === "Limited"
                ? "warn"
                : "bad")
            }
          >
            {kernelStatus.loading ? "Checking…" : kernelStatus()}
          </span>
        </div>
      </div>

      {/* Kernel Capabilities */}
      <h2>Kernel Capabilities</h2>

      <div class="card">
        {kernelCaps.loading ? (
          <p class="label">Evaluating kernel permissions…</p>
        ) : (
          <>
            <div class="row">
              <span class="label">Kernel inspection</span>
              <span class={"status " + (kernelCaps()![0] ? "good" : "bad")}>
                {kernelCaps()![0] ? "Enabled" : "Disabled"}
              </span>
            </div>

            <div class="row">
              <span class="label">Kernel modification</span>
              <span class={"status " + (kernelCaps()![1] ? "good" : "bad")}>
                {kernelCaps()![1] ? "Enabled" : "Disabled"}
              </span>
            </div>

            <p class="label" style={{ "margin-top": "12px" }}>
              {kernelExplanation.loading
                ? "Analyzing kernel environment…"
                : kernelExplanation()}
            </p>
          </>
        )}
      </div>

      {/* ADB */}
      <h2>ADB</h2>

      <div class="card">
        <div class="row">
          <span class="label">ADB status</span>
          <span
            class={
              "status " +
              (adbStatus() === "Device connected"
                ? "good"
                : adbStatus() === "Device unauthorized"
                ? "warn"
                : "bad")
            }
          >
            {adbStatus.loading ? "Checking…" : adbStatus()}
          </span>
        </div>
      </div>

      {/* Actions */}
      <h2>Actions</h2>

      <div class="card">
        <button class="action" disabled={!adbConnected()}>
          Reboot
        </button>

        <button class="action" disabled={!adbConnected()}>
          Recovery
        </button>

        <button class="action" disabled={!adbConnected()}>
          Bootloader
        </button>

        <button class="action" disabled>
          EDL (MTK)
        </button>

        <p class="label" style={{ "margin-top": "12px" }}>
          {adbConnected()
            ? "Actions enabled for connected device."
            : "Connect and authorize a device to enable actions."}
        </p>
      </div>
    </main>
  );
}

export default App;
