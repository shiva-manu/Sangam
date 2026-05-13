/**
 * @fileoverview Main landing page for the Sangam desktop app.
 *
 * The dashboard composes the primary cluster, node, task, log, and resource
 * panels into one operator-friendly overview.
 */
import { ClusterOverview } from "../components/dashboard/ClusterOverview";
import { MeshVisualization } from "../components/dashboard/MeshVisualization";
import { ActiveNodes } from "../components/dashboard/ActiveNodes";
import { TaskQueue } from "../components/dashboard/TaskQueue";
import { RuntimeConsole } from "../components/dashboard/RuntimeConsole";
import { ResourceGraphs } from "../components/dashboard/ResourceGraphs";

/**
 * Dashboard — the headline page.
 *
 * Composes all six self-contained sections into a single scrollable column.
 * Each section owns its data hooks, so adding or removing panels does not
 * ripple data-fetching concerns into the page itself.
 */
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

      {/* On wide screens, a 2-column grid lets topology and task flow be read
          together; on narrow screens they stack for comfortable scrolling. */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <MeshVisualization />
        <TaskQueue />
      </div>

      <ActiveNodes />

      {/* Another wide-screen 2-column grid pairs raw runtime events with the
          resource graphs that explain the same activity numerically. */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <RuntimeConsole />
        <ResourceGraphs />
      </div>
    </div>
  );
}
