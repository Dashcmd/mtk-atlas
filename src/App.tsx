import "./App.css";

import {
  createSignal,
  createEffect,
  onMount,
  onCleanup,
  Show,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

/* ================= TYPES ================= */

type DeviceState =
  | "Disconnected"
  | "AdbUnauthorized"
  | "AdbDevice"
  | "Fastboot"
  | "MtkPreloader";

type LogLevel = "info" | "warn" | "error";

type LogEntry = {
  ts: string;
  level: LogLevel;
  msg: string;
};

type InstallProgress = {
  stage: string;
  percent: number;
};

/* ================= APP ================= */

export default function App() {
  /* ============ THEME ============ */
  const [theme, setTheme] = createSignal<"light" | "dark">(
    (localStorage.getItem("mtk-theme") as "light" | "dark") || "light"
  );

  createEffect(() => {
    document.documentElement.setAttribute("data-theme", theme());
    localStorage.setItem("mtk-theme", theme());
  });

  /* ============ NAV ============ */
  const [page, setPage] =
    createSignal<"dashboard" | "commands" | "help">("dashboard");

  /* ============ CORE STATE ============ */
  const [deviceState, setDeviceState] =
    createSignal<DeviceState>("Disconnected");

  const [toolsInstalled, setToolsInstalled] =
    createSignal(false);

  const [installingTools, setInstallingTools] =
    createSignal(false);

  const [installProgress, setInstallProgress] =
    createSignal<InstallProgress | null>(null);

  const [useRoot, setUseRoot] = createSignal(false);

  /* ============ LOGGING ============ */
  const [logs, setLogs] = createSignal<LogEntry[]>([]);

  function pushLog(msg: string, level: LogLevel = "info") {
    setLogs(l => [
      ...l.slice(-300),
      { ts: new Date().toLocaleTimeString(), level, msg },
    ]);
  }

  /* ============ DERIVED FLAGS ============ */

  const adbPresent = () =>
    deviceState() === "AdbDevice" ||
    deviceState() === "AdbUnauthorized";

  const adbAuthorized = () =>
    deviceState() === "AdbDevice";

  const adbRunnable = () =>
    adbPresent() && toolsInstalled();

  const fastbootRunnable = () =>
    deviceState() === "Fastboot" && toolsInstalled();

  /* ================= EVENTS ================= */

  onMount(async () => {
    const unlistenDevice = await listen<DeviceState>(
      "device_state_changed",
      e => {
        setDeviceState(e.payload);
        pushLog(`Device state → ${e.payload}`);
      }
    );

    const unlistenInstall = await listen<InstallProgress>(
      "platform-tools-progress",
      e => {
        setInstallProgress(e.payload);
        pushLog(
          `Platform-tools: ${e.payload.stage} (${e.payload.percent}%)`
        );
      }
    ).catch(() => null);

    invoke<boolean>("platform_tools_installed_cmd")
      .then(setToolsInstalled)
      .catch(() => setToolsInstalled(false));

    onCleanup(() => {
      unlistenDevice();
      if (unlistenInstall) unlistenInstall();
    });
  });

  /* ================= PLATFORM TOOLS ================= */

  async function installPlatformTools() {
    if (installingTools() || toolsInstalled()) return;

    setInstallingTools(true);
    setInstallProgress(null);
    pushLog("Installing platform-tools…");

    try {
      await invoke("install_platform_tools_cmd");
      setToolsInstalled(true);
      pushLog("Platform-tools installed successfully");
    } catch (e) {
      pushLog(`Platform-tools install failed: ${e}`, "error");
    } finally {
      setInstallingTools(false);
    }
  }

  /* ================= ADB ================= */

  const [adbCmd, setAdbCmd] = createSignal("");
  const [adbOut, setAdbOut] = createSignal("");
  const [adbBusy, setAdbBusy] = createSignal(false);

  async function runAdb(cmd: string) {
    if (!cmd.trim() || adbBusy() || !adbRunnable()) return;

    setAdbBusy(true);
    setAdbOut("");

    const finalCmd = useRoot()
      ? `shell su -c "${cmd.replace(/"/g, '\\"')}"`
      : cmd;

    try {
      const out = await invoke<string>("adb_run", {
        command: finalCmd,
      });
      setAdbOut(out || "(no output)");
    } catch (e) {
      setAdbOut(String(e));
      pushLog(`ADB error: ${e}`, "error");
    } finally {
      setAdbBusy(false);
    }
  }

  /* ================= FASTBOOT ================= */

  const [fbCmd, setFbCmd] = createSignal("");
  const [fbOut, setFbOut] = createSignal("");
  const [fbBusy, setFbBusy] = createSignal(false);

  async function runFastboot(cmd: string) {
    if (!cmd.trim() || fbBusy() || !fastbootRunnable()) return;

    setFbBusy(true);
    setFbOut("");

    try {
      const out = await invoke<string>("fastboot_run", {
        command: cmd,
      });
      setFbOut(out || "(no output)");
    } catch (e) {
      setFbOut(String(e));
      pushLog(`Fastboot error: ${e}`, "error");
    } finally {
      setFbBusy(false);
    }
  }

  /* ================= DIAGNOSTICS ================= */

  async function exportDiagnostics() {
    try {
      const path = await invoke<string>("export_diagnostics", {
        logs: logs()
          .map(l => `[${l.ts}] ${l.level}: ${l.msg}`)
          .join("\n"),
        device_state: deviceState(),
      });
      pushLog(`Diagnostics exported → ${path}`);
    } catch (e) {
      pushLog(`Diagnostics export failed: ${e}`, "error");
    }
  }

  /* ================= UI ================= */

  return (
    <main class="container">
      <header class="header">
        <div>
          <h1>MTK Atlas</h1>
          <span class="subtitle">
            MediaTek device detection & control
          </span>
        </div>

        <button
          class="theme-toggle"
          onClick={() =>
            setTheme(theme() === "light" ? "dark" : "light")
          }
        >
          {theme() === "light" ? "Dark mode" : "Light mode"}
        </button>

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

      <Show when={page() === "dashboard"}>
        <section class="card">
          <strong>Device State:</strong> {deviceState()}
        </section>

        <Show when={!toolsInstalled()}>
          <section class="card warn">
            <strong>Platform-tools required</strong>
            <button onClick={installPlatformTools}>
              {installingTools()
                ? "Installing…"
                : "Install platform-tools"}
            </button>
            <Show when={installProgress()}>
              <div>
                {installProgress()!.stage} —{" "}
                {installProgress()!.percent}%
              </div>
            </Show>
          </section>
        </Show>

        <section class="card">
          <label>
            <input
              type="checkbox"
              checked={useRoot()}
              disabled={!adbAuthorized()}
              onChange={e =>
                setUseRoot(e.currentTarget.checked)
              }
            />{" "}
            Run ADB commands as root
          </label>
        </section>
      </Show>

      <Show when={page() === "commands"}>
        <section class="card">
          <h3>ADB</h3>
          <input
            disabled={!adbRunnable()}
            placeholder="adb command"
            value={adbCmd()}
            onInput={e => setAdbCmd(e.currentTarget.value)}
            onKeyDown={e =>
              e.key === "Enter" && runAdb(adbCmd())
            }
          />
          <pre class="terminal">
            {adbBusy() ? "Running…" : adbOut()}
          </pre>
        </section>

        <section
          class={`card ${deviceState() === "Fastboot" ? "fastboot" : ""}`}
        >
          <h3>Fastboot</h3>
          <input
            disabled={!fastbootRunnable()}
            placeholder="fastboot command"
            value={fbCmd()}
            onInput={e => setFbCmd(e.currentTarget.value)}
            onKeyDown={e =>
              e.key === "Enter" && runFastboot(fbCmd())
            }
          />
          <pre class="terminal">
            {fbBusy() ? "Running…" : fbOut()}
          </pre>
        </section>
      </Show>

      <Show when={page() === "help"}>
        <section class="card">
          <p>
            MTK Atlas allows unrestricted ADB and Fastboot
            commands. Destructive actions are your
            responsibility.
          </p>
          <button onClick={exportDiagnostics}>
            Export diagnostics
          </button>
        </section>
      </Show>

      <section class="card">
        <h3>Logs</h3>
        <pre class="terminal">
          {logs()
            .map(
              l =>
                `[${l.ts}] ${l.level}: ${l.msg}`
            )
            .join("\n")}
        </pre>
      </section>
    </main>
  );
}
