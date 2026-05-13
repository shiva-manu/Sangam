/**
 * @fileoverview Full-width active nodes page.
 *
 * Surfaces the `ActiveNodes` panel by itself so operators can inspect local and
 * peer device state without other dashboard panels around it.
 */
import { ActiveNodes } from "../components/dashboard/ActiveNodes";

/** Nodes page — every device on the mesh, with their per-node telemetry. */
export function Nodes() {
  return (
    <div className="max-w-[1600px] mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-semibold text-ink tracking-tight">
          Nodes
        </h1>
        <p className="text-sm text-ink-muted mt-1">
          Per-device state for every peer participating in this mesh
        </p>
      </header>
      <ActiveNodes />
    </div>
  );
}
