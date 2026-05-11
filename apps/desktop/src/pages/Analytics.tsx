import { ResourceGraphs } from "../components/dashboard/ResourceGraphs";

/// Analytics page — resource graphs. Will grow once we add historical
/// roll-ups and cross-peer aggregations.
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
