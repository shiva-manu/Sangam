/**
 * @fileoverview Top-level React Router configuration for the desktop app.
 *
 * This module defines Sangam's page routes and nests every route beneath the
 * persistent `AppShell` frame so navigation swaps only the page body.
 */
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AppShell } from "./components/layout/AppShell";
import { Dashboard } from "./pages/Dashboard";
import { Cluster } from "./pages/Cluster";
import { Nodes } from "./pages/Nodes";
import { Tasks } from "./pages/Tasks";
import { Runtime } from "./pages/Runtime";
import { Analytics } from "./pages/Analytics";
import { Settings } from "./pages/Settings";

/**
 * Top-level routed app.
 *
 * Route structure:
 * - `/` → Dashboard
 * - `/cluster` → focused topology view
 * - `/nodes` → active node inventory
 * - `/tasks` → task queue
 * - `/runtime` → live runtime console
 * - `/analytics` → resource graphs
 * - `/settings` → local configuration controls
 *
 * All routes nest inside `AppShell` so the sidebar and top bar persist across
 * navigations while only the page body re-renders and animates.
 */
function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route index element={<Dashboard />} />
          <Route path="cluster" element={<Cluster />} />
          <Route path="nodes" element={<Nodes />} />
          <Route path="tasks" element={<Tasks />} />
          <Route path="runtime" element={<Runtime />} />
          <Route path="analytics" element={<Analytics />} />
          <Route path="settings" element={<Settings />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
