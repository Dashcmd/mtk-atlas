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
  const [shellHistory, setShellHistory] = createSignal<string[]>([]);
  const [histIndex, setHistIndex] = createSignal(-1);
  const [useRoot, setUseRoot] = createSignal(false);

  /* === LOGCAT STATE === */
  const [logcatOut, setLogcatOut] = createSignal("");
  const [logcatRunning, setLogcatRunning] = createSignal(false);

  /* === AUTHORITATIVE READY FLAGS === */
  const adbReady = () => deviceState() === "AdbDevice";
  const fastbootReady = () => deviceState() === "Fastboot";
  const preloaderReady = () => deviceState() === "MtkPreloader";

  /* ================= LOGGING ================= */

  function log(msg: string, level: LogLevel = "info") {
    setLogs(l => [
      ...l.slice(-200), // cap log growth
      { ts: new Date().toLocaleTimeString(), level, msg },
    ]);
  }

  function logError(e: unknown, context: string) {
    const msg =
      e instanceof Error ? e.message : typeof e === "string" ? e : "Unknown error";
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
        setTimeout(() => setAnimating(false), 250);
      }
    );

    onCleanup(() => unlisten());
  });

  /* ================= DATA FETCH (NO POLLING) ================= */

  const [deviceInfo] = createResource(
    () => adbReady(),
    async ready => {
      if (!ready) return null;
      try {
        return await invoke<[string, string] | null>(
          "get_adb_device_info"
        );
      } catch (e) {
        logError(e, "Device info fetch failed");
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

  /* ================= SHELL EXECUTION ================= */

  async function runShell(cmd: string) {
    if (!cmd.trim() || !adbReady()) return;

    setShellBusy(true);
    setShellOut("");

    const finalCmd = useRoot()
      ? `su -c "${cmd.replace(/"/g, '\\"')}"`
      : cmd;

    setShellHistory(h => [...h, cmd]);
    setHistIndex(-1);

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

  /* ================= LOGCAT ================= */

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
        <h1>MTK Atlas</h1>
        <span class="subtitle">
          Universal MediaTek control & analysis
        </span>
      </header>

      {/* === STATUS GRID === */}
      <section class="grid">
        <div class="card">
          <h3>Connection</h3>
          <div class={`status ${adbReady() ? "good" : "bad"}`}>
            ADB: {deviceState()}
          </div>
          <div class={`status ${fastbootReady() ? "good" : "neutral"}`}>
            Fastboot: {fastbootReady() ? "Connected" : "Idle"}
          </div>
          <div class={`status ${preloaderReady() ? "warn" : "neutral"}`}>
            MTK Preloader: {preloaderReady() ? "Detected" : "Idle"}
          </div>
        </div>

        <div class="card">
          <h3>Device</h3>
          {deviceInfo() ? (
            <>
              <div>
                <strong>Model:</strong> {deviceInfo()![0]}
              </div>
              <div>
                <strong>Serial:</strong> {deviceInfo()![1]}
              </div>
            </>
          ) : (
            <div class="muted">No device</div>
          )}
        </div>

        <div class="card">
          <h3>Capabilities</h3>
          <pre class="caps">
            {JSON.stringify(mtkCaps(), null, 2)}
          </pre>
        </div>
      </section>

      {/* === ADB SHELL === */}
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

        <div style={{ "margin-top": "0.5rem" }}>
          <label>
            <input
              type="checkbox"
              disabled={!adbReady()}
              checked={useRoot()}
              onChange={e => setUseRoot(e.currentTarget.checked)}
            />{" "}
            Run as root (su)
          </label>
        </div>

        <pre class="terminal">
          {shellBusy() ? "Running…" : shellOut()}
        </pre>
      </section>

      {/* === LOGCAT === */}
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

      {/* === LOG PANEL === */}
      <section class="card log">
        <h3>Activity Log</h3>
        <div class="log-body">
          {logs().length === 0 && (
            <div class="muted">No activity yet</div>
          )}
          {logs().map(l => (
            <div class={`log-line ${l.level}`}>
              <span class="ts">{l.ts}</span>
              <span class="msg">{l.msg}</span>
            </div>
          ))}
        </div>
      </section>
    </main>
  );
}

export default App;
