import { createResource } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [kernelStatus] = createResource(() =>
    invoke<string>("get_kernel_status")
  );

  const [deviceProfile] = createResource(() =>
    invoke<string>("get_active_profile")
  );

  return (
    <main class="container">
      <h1>MTK Atlas</h1>

      <p>
        <strong>Active Device Profile:</strong>{" "}
        {deviceProfile.loading ? "Detecting…" : deviceProfile()}
      </p>

      <p>
        <strong>Kernel Status:</strong>{" "}
        {kernelStatus.loading ? "Checking…" : kernelStatus()}
      </p>
    </main>
  );
}

export default App;
