import { RuntimeConsole } from "../components/dashboard/RuntimeConsole";

/// Runtime page — full-width log console.
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
