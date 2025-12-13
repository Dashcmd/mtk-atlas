import {
  createResource,
  createSignal,
  onCleanup,
  onMount,
  createEffect,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

/* ================= TYPES ================= */

type LogLevel = "info" | "warn" | "error";

type DeviceState =
  | "Disconnected"
  | "AdbUnauthorized"
  | "AdbDevice"
  | "Fastboot"
  | "MtkPreloader";

/* ================= APP ================= */

function App() {
  /* === PAGE NAV === */
  const [page, setPage] = createSignal<
    "dashboard" | "help" | "commands"
  >("dashboard");

  /* === CORE STATE === */
  const [deviceState, setDeviceState] =
    createSignal<DeviceState>("Disconnected");

  const [logs, setLogs] = createSignal<
    { ts: string; level: LogLevel; msg: string }[]
  >([]);

  const [animating, setAnimating] = createSignal(false);

  /* === SHELL STATE === */
  const [shellCmd, setShellCmd] = createSignal("");
  const [shellOut, setShellOut] = createSignal("");
  const [shellBusy, setShellBusy] = createSignal(false);
  const [useRoot, setUseRoot] = createSignal(false);

  /* === LOGCAT STATE === */
  const [logcatOut, setLogcatOut] = createSignal("");
  const [logcatRunning, setLogcatRunning] = createSignal(false);

  /* === READY FLAGS === */
  const adbReady = () => deviceState() === "AdbDevice";
  const fastbootReady = () => deviceState() === "Fastboot";
  const preloaderReady = () => deviceState() === "MtkPreloader";

  /* ================= LOGGING ================= */

  function log(msg: string, level: LogLevel = "info") {
    setLogs(l => [
      ...l.slice(-200),
      { ts: new Date().toLocaleTimeString(), level, msg },
    ]);
  }

  function logError(e: unknown, context: string) {
    const msg =
      e instanceof Error
        ? e.message
        : typeof e === "string"
        ? e
        : "Unknown error";
    log(`${context}: ${msg}`, "error");
  }

  /* ================= STATE PERSISTENCE ================= */

  onMount(() => {
    const saved = localStorage.getItem("mtk-atlas:last-device-state");
    if (saved) {
      setDeviceState(saved as DeviceState);
      log(`Restored device state → ${saved}`, "warn");
    }
  });

  createEffect(() => {
    localStorage.setItem(
      "mtk-atlas:last-device-state",
      deviceState()
    );
  });

  /* ================= BACKEND EVENTS ================= */

  onMount(async () => {
    const unlisten = await listen<DeviceState>(
      "device_state_changed",
      event => {
        setAnimating(true);
        setDeviceState(event.payload);
        log(`Device state → ${event.payload}`);
        setTimeout(() => setAnimating(false), 200);
      }
    );
    onCleanup(() => unlisten());
  });

  /* ================= DATA ================= */

  const [deviceInfo] = createResource(
    () => adbReady(),
    async ready => {
      if (!ready) return null;
      try {
        return await invoke<[string, string] | null>(
          "get_adb_device_info"
        );
      } catch (e) {
        logError(e, "Device info failed");
        return null;
      }
    }
  );

  const [mtkCaps] = createResource(async () => {
    try {
      return await invoke<any>("get_mtk_capabilities");
    } catch (e) {
      logError(e, "Capability query failed");
      return null;
    }
  });

  /* ================= ACTIONS ================= */

  async function runShell(cmd: string) {
    if (!cmd.trim() || !adbReady()) return;

    setShellBusy(true);
    setShellOut("");

    const finalCmd = useRoot()
      ? `su -c "${cmd.replace(/"/g, '\\"')}"`
      : cmd;

    try {
      const out = await invoke<string>("adb_shell", {
        command: finalCmd,
      });
      setShellOut(out || "(no output)");
    } catch (e) {
      logError(e, "ADB shell failed");
      setShellOut(String(e));
    } finally {
      setShellBusy(false);
    }
  }

  async function startLogcat() {
    if (!adbReady()) return;

    setLogcatRunning(true);
    setLogcatOut("");

    try {
      const out = await invoke<string>("adb_shell", {
        command: "logcat -d",
      });
      setLogcatOut(out);
    } catch (e) {
      logError(e, "Logcat failed");
      setLogcatOut(String(e));
    } finally {
      setLogcatRunning(false);
    }
  }

  /* ================= UI ================= */

  return (
    <main class={`container ${animating() ? "fade" : ""}`}>
      <header class="header">
        <div>
          <h1>MTK Atlas</h1>
          <span class="subtitle">
            MediaTek device detection & control
          </span>
        </div>

        <div class="nav">
          <button
            class={page() === "dashboard" ? "active" : ""}
            onClick={() => setPage("dashboard")}
          >
            Dashboard
          </button>
          <button
            class={page() === "commands" ? "active" : ""}
            onClick={() => setPage("commands")}
          >
            Commands
          </button>
          <button
            class={page() === "help" ? "active" : ""}
            onClick={() => setPage("help")}
          >
            Help
          </button>
        </div>
      </header>

      {/* ================= DASHBOARD ================= */}
      {page() === "dashboard" && (
        <>
          <section class="grid">
            <div class="card primary">
              <h3>Connection Status</h3>
              <div class={`status-box ${adbReady() ? "good" : "bad"}`}>
                ADB: {deviceState()}
              </div>
              <div class={`status-box ${fastbootReady() ? "good" : "neutral"}`}>
                Fastboot: {fastbootReady() ? "Connected" : "Idle"}
              </div>
              <div class={`status-box ${preloaderReady() ? "warn" : "neutral"}`}>
                MTK Preloader: {preloaderReady() ? "Detected" : "Idle"}
              </div>
            </div>

            <div class="card">
              <h3>Connected Device</h3>
              {deviceInfo() ? (
                <>
                  <div><strong>Model:</strong> {deviceInfo()![0]}</div>
                  <div><strong>Serial:</strong> {deviceInfo()![1]}</div>
                </>
              ) : (
                <div class="muted">No device connected</div>
              )}
            </div>

            <div class="card">
              <h3>MTK Capabilities</h3>
          <div class="caps-grid">
  <div class={`cap ${mtkCaps()?.adb ? "ok" : "off"}`}>
    <span class="cap-title">ADB</span>
    <span class="cap-desc">
      {mtkCaps()?.adb ? "Available" : "Unavailable"}
    </span>
  </div>

  <div class={`cap ${mtkCaps()?.fastboot ? "ok" : "off"}`}>
    <span class="cap-title">Fastboot</span>
    <span class="cap-desc">
      {mtkCaps()?.fastboot ? "Available" : "Unavailable"}
    </span>
  </div>

  <div class={`cap ${mtkCaps()?.preloader ? "warn" : "off"}`}>
    <span class="cap-title">Preloader</span>
    <span class="cap-desc">
      {mtkCaps()?.preloader ? "Detected" : "Idle"}
    </span>
  </div>

  <div class={`cap ${mtkCaps()?.brom ? "danger" : "off"}`}>
    <span class="cap-title">BROM</span>
    <span class="cap-desc">
      {mtkCaps()?.brom ? "Active" : "Inactive"}
    </span>
  </div>
</div>

<div class="caps-desc">
  {mtkCaps()?.description}
</div>

            </div>
          </section>

          <section class="card">
            <h3>ADB Shell</h3>
            <textarea
              rows={3}
              placeholder="Enter adb shell command"
              disabled={!adbReady()}
              value={shellCmd()}
              onInput={e => setShellCmd(e.currentTarget.value)}
              onKeyDown={e => {
                if (e.key === "Enter" && !e.shiftKey) {
                  e.preventDefault();
                  runShell(shellCmd());
                  setShellCmd("");
                }
              }}
            />
            <label>
              <input
                type="checkbox"
                disabled={!adbReady()}
                checked={useRoot()}
                onChange={e => setUseRoot(e.currentTarget.checked)}
              />{" "}
              Run as root (su)
            </label>
            <pre class="terminal">
              {shellBusy() ? "Running…" : shellOut()}
            </pre>
          </section>

          <section class="card">
            <h3>Logcat</h3>
            <button
              disabled={!adbReady() || logcatRunning()}
              onClick={startLogcat}
            >
              {logcatRunning() ? "Collecting…" : "Fetch logcat"}
            </button>
            <pre class="terminal">{logcatOut()}</pre>
          </section>
        </>
      )}

      {/* ================= COMMANDS ================= */}
      {page() === "commands" && (
        <section class="help">
          <h2>Common Commands Reference</h2>

          <div class="block">
            <h4>ADB Commands (Android Running)</h4>
            <p><code>adb devices</code> — List connected Android devices</p>
            <p><code>adb shell</code> — Open a shell on the device</p>
            <p><code>adb reboot</code> — Reboot the device normally</p>
            <p><code>adb reboot recovery</code> — Reboot into recovery</p>
            <p><code>adb reboot bootloader</code> — Reboot into fastboot</p>
            <p><code>adb logcat</code> — View Android system logs</p>
          </div>

          <div class="block">
            <h4>Fastboot Commands (Bootloader Mode)</h4>
            <p><code>fastboot devices</code> — List fastboot devices</p>
            <p><code>fastboot getvar all</code> — Show device variables</p>
            <p><code>fastboot reboot</code> — Reboot device</p>
            <p><code>fastboot reboot-bootloader</code> — Restart fastboot</p>
            <p><code>fastboot getvar current-slot</code> — Show active slot</p>
          </div>

          <div class="block">
            <h4>MTK / Preloader Notes</h4>
            <p>
              MediaTek preloader (BROM) operations are extremely low-level.
              MTK Atlas intentionally limits access to prevent accidental
              damage. Advanced features will be clearly marked.
            </p>
          </div>
        </section>
      )}

      {/* ================= HELP ================= */}
      {page() === "help" && (
        <section class="help">
          <h2>MTK Atlas Help</h2>

          <div class="block">
            <h4>ADB (Android Debug Bridge)</h4>
            <p>
              Used when Android is booted. Provides shell access, logcat,
              and reboot control. Requires authorization on the device.
            </p>
          </div>

          <div class="block">
            <h4>Fastboot Mode</h4>
            <p>
              Bootloader mode used for flashing and slot management.
              Android is not running in this state.
            </p>
          </div>

          <div class="block">
            <h4>MTK Preloader / BROM</h4>
            <p>
              The earliest MediaTek boot stage. Very powerful and dangerous.
              MTK Atlas enforces safety limits by design.
            </p>
          </div>

          <div class="block">
            <h4>Safety Model</h4>
            <p>
              Commands are only enabled when the device is in the correct
              state to avoid soft bricks or data loss.
            </p>
          </div>
        </section>
      )}
    </main>
  );
}

export default App;
