/**
 * @fileoverview Full-width runtime console page.
 *
 * Gives users a focused live log-tail view when debugging runtime behaviour or
 * watching the mesh negotiate tasks and discovery events.
 */
import { RuntimeConsole } from "../components/dashboard/RuntimeConsole";

/** Runtime page — full-width log console for focused live-tail monitoring. */
export function Runtime() {
  return (
    <div className="max-w-[1400px] mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-semibold text-ink tracking-tight">
          Runtime
        </h1>
        <p className="text-sm text-ink-muted mt-1">
          Live tail of every event the runtime emits
        </p>
      </header>
      <RuntimeConsole />
    </div>
  );
}
