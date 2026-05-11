import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AppShell } from "./components/layout/AppShell";
import { Dashboard } from "./pages/Dashboard";
import { Cluster } from "./pages/Cluster";
import { Nodes } from "./pages/Nodes";
import { Tasks } from "./pages/Tasks";
import { Runtime } from "./pages/Runtime";
import { Analytics } from "./pages/Analytics";
import { Settings } from "./pages/Settings";

/// Top-level routed app. Every page renders inside `AppShell` so the
/// sidebar + top bar persist across navigations and only the page body
/// animates in/out.
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
