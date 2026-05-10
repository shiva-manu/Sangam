import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type NodeInfo = {
  local_ip: string;
  port: number;
  running: boolean;
};

function App() {
  const [info, setInfo] = useState<NodeInfo | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    try {
      const next = await invoke<NodeInfo>("get_node_info");
      setInfo(next);
      setError(null);
    } catch (e) {
      setError(String(e));
    }
  }

  // Poll node info every 2s — cheap, and gives the dashboard a live feel.
  useEffect(() => {
    refresh();
    const id = setInterval(refresh, 2000);
    return () => clearInterval(id);
  }, []);

  async function start() {
    setBusy(true);
    try {
      await invoke("start_runtime");
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  async function stop() {
    setBusy(true);
    try {
      await invoke("stop_runtime");
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  const running = info?.running ?? false;

  return (
    <main className="dashboard">
      <header className="header">
        <div className="brand">
          <span className="logo">⌬</span>
          <h1>Sangam</h1>
          <span className="tagline">control plane</span>
        </div>
        <div className={`status-pill ${running ? "on" : "off"}`}>
          <span className="dot" />
          {running ? "RUNNING" : "STOPPED"}
        </div>
      </header>

      <section className="grid">
        <article className="card">
          <h2>Node</h2>
          <dl>
            <dt>Local IP</dt>
            <dd>{info?.local_ip ?? "—"}</dd>
            <dt>Port</dt>
            <dd>{info?.port ?? "—"}</dd>
            <dt>State</dt>
            <dd>{running ? "running" : "idle"}</dd>
          </dl>
        </article>

        <article className="card">
          <h2>Runtime</h2>
          <p className="muted">
            Start the runtime to advertise this node over mDNS and accept
            tasks from peers on the local network.
          </p>
          <div className="actions">
            <button
              className="primary"
              onClick={start}
              disabled={busy || running}
            >
              Start
            </button>
            <button
              className="ghost"
              onClick={stop}
              disabled={busy || !running}
            >
              Stop
            </button>
          </div>
        </article>

        <article className="card placeholder">
          <h2>Peers</h2>
          <p className="muted">
            Discovered peers will appear here once peer-event streaming is
            wired up. Today the runtime logs them to stdout.
          </p>
        </article>

        <article className="card placeholder">
          <h2>Tasks</h2>
          <p className="muted">
            Submitted tasks and results will surface here. Coming next.
          </p>
        </article>
      </section>

      {error && (
        <footer className="error" role="alert">
          {error}
        </footer>
      )}
    </main>
  );
}

export default App;
