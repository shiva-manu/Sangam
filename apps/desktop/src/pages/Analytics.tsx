/**
 * @fileoverview Analytics page for resource and task trends.
 *
 * This page will grow to include cross-peer aggregations and historical roll-ups;
 * for now it wraps `ResourceGraphs` as the dedicated graph-focused route.
 */
import { ResourceGraphs } from "../components/dashboard/ResourceGraphs";

/** Analytics page — currently resource graphs, later historical mesh analytics. */
export function Analytics() {
  return (
    <div className="max-w-[1400px] mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-semibold text-ink tracking-tight">
          Analytics
        </h1>
        <p className="text-sm text-ink-muted mt-1">
          CPU, RAM, network, and task throughput over time
        </p>
      </header>
      <ResourceGraphs />
    </div>
  );
}
