import { ClusterOverview } from "../components/dashboard/ClusterOverview";
import { MeshVisualization } from "../components/dashboard/MeshVisualization";
import { ActiveNodes } from "../components/dashboard/ActiveNodes";
import { TaskQueue } from "../components/dashboard/TaskQueue";
import { RuntimeConsole } from "../components/dashboard/RuntimeConsole";
import { ResourceGraphs } from "../components/dashboard/ResourceGraphs";

/// Dashboard — the headline page. Composes all six sections into a single
/// scrollable column. Each section is self-contained (own data hooks),
/// so adding/removing sections doesn't ripple into the page itself.
export function Dashboard() {
  return (
    <div className="max-w-[1600px] mx-auto space-y-6">
      {/* Page heading */}
      <div className="flex items-baseline justify-between">
        <div>
          <h1 className="text-2xl font-semibold text-ink tracking-tight">
            Dashboard
          </h1>
          <p className="text-sm text-ink-muted mt-1">
            Live overview of the local Sangam mesh
          </p>
        </div>
      </div>

      <ClusterOverview />

      {/* Mesh + task queue side by side on wider screens */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <MeshVisualization />
        <TaskQueue />
      </div>

      <ActiveNodes />

      {/* Runtime console + resource graphs side by side */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <RuntimeConsole />
        <ResourceGraphs />
      </div>
    </div>
  );
}
