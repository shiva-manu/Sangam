/**
 * @fileoverview Full-width task queue page.
 *
 * Surfaces `TaskQueue` for operators monitoring distributed task throughput,
 * lifecycle transitions, and peer assignment details.
 */
import { TaskQueue } from "../components/dashboard/TaskQueue";

/** Tasks page — full-width task queue for operator throughput monitoring. */
export function Tasks() {
  return (
    <div className="max-w-[1200px] mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-semibold text-ink tracking-tight">
          Tasks
        </h1>
        <p className="text-sm text-ink-muted mt-1">
          Lifecycle of every compute task flowing through this node
        </p>
      </header>
      <TaskQueue />
    </div>
  );
}
