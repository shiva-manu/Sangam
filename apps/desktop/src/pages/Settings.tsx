/**
 * @fileoverview Settings page for mesh contribution and policy controls.
 *
 * The UI is intentionally wired with local state while the backend policy API
 * is still evolving; it establishes the UX shape for future runtime settings.
 */
import { useState } from "react";
import {
  BatteryCharging,
  ChevronRight,
  Lock,
  Network,
  Radar,
  ShieldCheck,
  SlidersHorizontal,
} from "lucide-react";
import { Card, CardHeader, CardTitle } from "../components/ui/Card";
import { Switch } from "../components/ui/Switch";
import { Slider } from "../components/ui/Slider";
import { Badge } from "../components/ui/Badge";

/**
 * Settings page.
 *
 * All controls are local-only for now. Once the backend grows a `RuntimePolicy`
 * struct plus Tauri commands to read/write it, each `useState` pair can be
 * replaced or hydrated by the real API.  For now the page locks in UX shape.
 */
export function Settings() {
  // Local-only settings state: no persistence yet because the Rust backend does
  // not currently expose a `RuntimePolicy` struct or policy mutation commands.
  const [cpuLimit, setCpuLimit] = useState(60);
  const [batteryMode, setBatteryMode] = useState(true);
  const [isolation, setIsolation] = useState(true);
  const [autoDiscover, setAutoDiscover] = useState(true);
  const [networkMode, setNetworkMode] = useState<"lan" | "manual">("lan");
  const [whitelist, setWhitelist] = useState<string[]>([
    "lab-mbp-01",
    "rig-linux-02",
  ]);

  return (
    <div className="max-w-[960px] mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-semibold text-ink tracking-tight">
          Settings
        </h1>
        <p className="text-sm text-ink-muted mt-1">
          Configure how this device contributes to the mesh
        </p>
      </header>

      {/* CPU limits */}
      <Card>
        <CardHeader>
          <div>
            <CardTitle className="flex items-center gap-2">
              <SlidersHorizontal className="w-3.5 h-3.5 text-accent-cyan" />
              Contribution
            </CardTitle>
            <p className="text-xs text-ink-muted mt-1">
              How much of this device can the mesh use?
            </p>
          </div>
        </CardHeader>

        <div className="space-y-5">
          <Field
            title="CPU contribution limit"
            description="Cap the fraction of CPU time the runtime is allowed to consume"
          >
            <div className="w-full max-w-xs">
              <div className="flex items-center justify-between mb-2">
                <span className="text-xs text-ink-muted">Max</span>
                <span className="font-mono text-sm text-ink tabular-nums">
                  {cpuLimit}%
                </span>
              </div>
              <Slider
                value={[cpuLimit]}
                min={10}
                max={100}
                step={5}
                onValueChange={(v) => setCpuLimit(v[0] ?? cpuLimit)}
              />
            </div>
          </Field>

          <Field
            title="Battery-aware mode"
            description="Pause contributions when running on battery below 20%"
          >
            <Switch
              checked={batteryMode}
              onCheckedChange={setBatteryMode}
              aria-label="Battery-aware mode"
            />
          </Field>
        </div>
      </Card>

      {/* Security & isolation */}
      <Card>
        <CardHeader>
          <div>
            <CardTitle className="flex items-center gap-2">
              <ShieldCheck className="w-3.5 h-3.5 text-accent-green" />
              Security
            </CardTitle>
            <p className="text-xs text-ink-muted mt-1">
              Sandbox and trust controls for incoming work
            </p>
          </div>
        </CardHeader>

        <div className="space-y-5">
          <Field
            title="Runtime isolation"
            description="Execute peer tasks inside a sandboxed worker process"
          >
            <Switch
              checked={isolation}
              onCheckedChange={setIsolation}
              aria-label="Runtime isolation"
            />
          </Field>

          <Field
            title="Trusted devices"
            description={`Only accept tasks from these peers (${whitelist.length} configured)`}
          >
            {/* Whitelist management is local/demo-only: remove filters by index,
                while add appends a generated placeholder device name until the
                backend provides durable trusted-device APIs. */}
            <div className="flex flex-col gap-2 min-w-[280px]">
              {whitelist.map((name, i) => (
                <div
                  key={name}
                  className="group flex items-center justify-between rounded-md bg-white/[0.03] border border-white/[0.06] px-2.5 py-1.5"
                >
                  <div className="flex items-center gap-2">
                    <Lock className="w-3 h-3 text-accent-green" />
                    <span className="font-mono text-xs text-ink">{name}</span>
                  </div>
                  <button
                    onClick={() =>
                      setWhitelist((prev) => prev.filter((_, j) => j !== i))
                    }
                    className="text-[10px] text-ink-dim hover:text-accent-red"
                  >
                    remove
                  </button>
                </div>
              ))}
              <button
                className="text-[11px] text-accent-cyan hover:text-accent-cyan/80 self-start"
                onClick={() =>
                  setWhitelist((prev) => [
                    ...prev,
                    `device-${(prev.length + 1).toString().padStart(2, "0")}`,
                  ])
                }
              >
                + Add device
              </button>
            </div>
          </Field>
        </div>
      </Card>

      {/* Network */}
      <Card>
        <CardHeader>
          <div>
            <CardTitle className="flex items-center gap-2">
              <Network className="w-3.5 h-3.5 text-accent-violet" />
              Network
            </CardTitle>
            <p className="text-xs text-ink-muted mt-1">
              How this device finds and reaches peers
            </p>
          </div>
        </CardHeader>

        <div className="space-y-5">
          <Field
            title="Auto-discovery"
            description="Broadcast presence over mDNS on the local network"
          >
            <Switch
              checked={autoDiscover}
              onCheckedChange={setAutoDiscover}
              aria-label="Auto-discovery"
            />
          </Field>

          <Field
            title="Network mode"
            description="How the mesh connects across hosts"
          >
            <div className="flex items-center gap-1 p-0.5 rounded-md bg-white/[0.03] border border-white/[0.06]">
              {(["lan", "manual"] as const).map((mode) => (
                <button
                  key={mode}
                  onClick={() => setNetworkMode(mode)}
                  className={
                    "px-3 py-1 text-xs rounded transition-colors " +
                    (networkMode === mode
                      ? "bg-white/[0.06] text-ink"
                      : "text-ink-dim hover:text-ink-muted")
                  }
                >
                  {mode === "lan" ? "Local LAN" : "Manual"}
                </button>
              ))}
            </div>
          </Field>

          <Field
            title="Discovery interval"
            description={`Re-broadcast presence every ${autoDiscover ? 2 : "—"} seconds`}
          >
            <Badge tone={autoDiscover ? "cyan" : "neutral"}>
              <Radar className="w-2.5 h-2.5" />
              {autoDiscover ? "Active" : "Disabled"}
            </Badge>
          </Field>
        </div>
      </Card>

      {/* About */}
      <Card>
        <CardHeader>
          <div>
            <CardTitle className="flex items-center gap-2">
              <BatteryCharging className="w-3.5 h-3.5 text-accent-amber" />
              About
            </CardTitle>
          </div>
        </CardHeader>
        <div className="space-y-2 text-xs">
          <Row label="Version" value="0.1.0 (development)" />
          <Row label="Runtime" value="Tokio + mDNS + TCP" />
          <Row label="Repository" value="github.com/shiva-manu/Sangam" />
        </div>
      </Card>
    </div>
  );
}

// Field helper: shared two-column settings row with label/description on the
// left and the interactive control aligned on the right.
function Field({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-start justify-between gap-6 py-1">
      <div className="min-w-0 flex-1">
        <div className="text-sm text-ink">{title}</div>
        {description && (
          <div className="text-xs text-ink-muted mt-0.5">{description}</div>
        )}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

// Row helper: compact read-only metadata row used by the About card.
function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between border-b border-white/[0.04] py-1.5 last:border-0">
      <span className="text-ink-muted">{label}</span>
      <span className="font-mono text-ink flex items-center gap-1">
        {value}
        <ChevronRight className="w-3 h-3 text-ink-dim" />
      </span>
    </div>
  );
}
