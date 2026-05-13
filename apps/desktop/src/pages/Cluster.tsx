/**
 * @fileoverview Focused cluster topology page.
 *
 * Reuses `ClusterOverview` and `MeshVisualization` for operators who want a
 * topology/capacity view without task, log, or chart panels competing for space.
 */
import { ClusterOverview } from "../components/dashboard/ClusterOverview";
import { MeshVisualization } from "../components/dashboard/MeshVisualization";

/** Cluster page — same data as Dashboard but focused on topology + totals. */
export function Cluster() {
  return (
    <div className="max-w-[1600px] mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-semibold text-ink tracking-tight">
          Cluster
        </h1>
        <p className="text-sm text-ink-muted mt-1">
          Self-organising mesh topology and aggregate capacity
        </p>
      </header>
      <ClusterOverview />
      <MeshVisualization />
    </div>
  );
}
