/**
 * @fileoverview High-level cluster capacity and health summary cards.
 *
 * This panel turns live peer, metrics, and task-count data into compact
 * aggregate indicators for the dashboard's headline row.
 */
import { motion } from "framer-motion";
import {
  Activity,
  Cpu,
  HardDrive,
  ListChecks,
  Radio,
  Users,
} from "lucide-react";
import {
  useMetrics,
  usePeers,
  useStatusCounts,
} from "../../hooks/use-runtime-data";
import { cn } from "../../lib/cn";
import { formatMib } from "../../lib/format";
import type { LucideIcon } from "lucide-react";

/**
 * Section 1 — Cluster Overview.
 *
 * Six high-density stat cards animate in on mount and update live.  Each card
 * carries a tiny accent gradient so the row reads like a set of instruments
 * rather than a uniform grid of numbers.
 */
export function ClusterOverview() {
  const { data: peers } = usePeers();
  const { data: metrics } = useMetrics();
  const { data: counts } = useStatusCounts();

  // Aggregate computations include known peers plus this device. `totalNodes`
  // adds self; CPU threads and RAM come from optional mDNS TXT properties, so
  // missing values contribute 0 rather than implying live telemetry.
  const totalNodes = peers.length + 1;
  const activeWorkers = peers.length; // peers are "workers" we can dispatch to
  const totalThreads = peers.reduce((acc, p) => acc + (p.cpu_threads ?? 0), 0);
  const totalRamMb = peers.reduce((acc, p) => acc + (p.ram_gib ?? 0) * 1024, 0);

  // `avgFreshnessMs` is labelled as latency but is really a heartbeat-age
  // proxy. It tells users how fresh peer discovery is without inventing RTT
  // measurements we do not yet collect.
  const now = Date.now();
  const avgFreshnessMs =
    peers.length === 0
      ? 0
      : peers.reduce((acc, p) => acc + (now - p.last_seen_ms), 0) /
        peers.length;

  const runningTasks = counts.running + counts.queued;

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3">
      <StatCard
        index={0}
        icon={Users}
        label="Total Nodes"
        value={String(totalNodes)}
        sub="incl. this device"
        tone="cyan"
      />
      <StatCard
        index={1}
        icon={Activity}
        label="Active Workers"
        value={String(activeWorkers)}
        sub={activeWorkers > 0 ? "online" : "discovering…"}
        tone="green"
      />
      <StatCard
        index={2}
        icon={Cpu}
        label="CPU Threads"
        value={String(totalThreads)}
        sub="shared"
        tone="violet"
      />
      <StatCard
        index={3}
        icon={HardDrive}
        label="RAM Shared"
        value={formatMib(totalRamMb)}
        sub="across peers"
        tone="amber"
      />
      <StatCard
        index={4}
        icon={Radio}
        label="Latency"
        value={
          peers.length === 0 ? "—" : `${Math.round(avgFreshnessMs / 1000)}s`
        }
        sub="peer freshness"
        tone="cyan"
      />
      <StatCard
        index={5}
        icon={ListChecks}
        label="Running Tasks"
        value={String(runningTasks)}
        sub={`${counts.completed} completed`}
        tone="violet"
        pulse={runningTasks > 0}
      />
      {/* CpuHeartbeat: a decorative pressure strip across the row. Hidden when
          the runtime has not produced metrics yet. */}
      {metrics && (
        <div className="col-span-full">
          <CpuHeartbeat cpuPct={metrics.cpu_pct} />
        </div>
      )}
    </div>
  );
}

// Visual tokens for each stat tone: background bloom (`glow`), icon colour,
// and card ring tint. Keeping them mapped here makes new tones easy to audit.
const toneStyles: Record<
  "cyan" | "violet" | "green" | "amber",
  { glow: string; icon: string; ring: string }
> = {
  cyan: {
    glow: "from-accent-cyan/15",
    icon: "text-accent-cyan",
    ring: "border-accent-cyan/20",
  },
  violet: {
    glow: "from-accent-violet/15",
    icon: "text-accent-violet",
    ring: "border-accent-violet/20",
  },
  green: {
    glow: "from-accent-green/15",
    icon: "text-accent-green",
    ring: "border-accent-green/20",
  },
  amber: {
    glow: "from-accent-amber/15",
    icon: "text-accent-amber",
    ring: "border-accent-amber/20",
  },
};

function StatCard({
  index,
  icon: Icon,
  label,
  value,
  sub,
  tone,
  pulse,
}: {
  index: number;
  icon: LucideIcon;
  label: string;
  value: string;
  sub?: string;
  tone: keyof typeof toneStyles;
  pulse?: boolean;
}) {
  const t = toneStyles[tone];
  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.4, delay: index * 0.04, ease: "easeOut" }}
      whileHover={{ y: -2 }}
      className={cn(
        "glass relative p-4 overflow-hidden group",
        "hover:border-white/[0.1] transition-colors",
      )}
    >
      {/* Tone-tinted radial bloom in top-right corner */}
      <div
        className={cn(
          "absolute -top-12 -right-12 w-32 h-32 rounded-full blur-3xl opacity-60",
          "bg-gradient-radial",
          t.glow,
          "to-transparent",
        )}
        aria-hidden
      />

      <div className="relative flex items-start justify-between mb-3">
        <span className="text-[10px] uppercase tracking-wider text-ink-dim font-medium">
          {label}
        </span>
        <div className={cn("p-1.5 rounded-md bg-white/[0.03] border", t.ring)}>
          <Icon className={cn("w-3.5 h-3.5", t.icon)} strokeWidth={2} />
        </div>
      </div>

      <div className="relative flex items-baseline gap-2">
        <div className="text-2xl font-semibold text-ink tracking-tight tabular-nums">
          {value}
        </div>
        {pulse && (
          <span className="relative inline-block w-1.5 h-1.5 rounded-full bg-accent-violet">
            <span className="absolute inset-0 rounded-full bg-accent-violet animate-ping-soft" />
          </span>
        )}
      </div>
      {sub && (
        <div className="relative text-[11px] text-ink-muted mt-1">{sub}</div>
      )}
    </motion.div>
  );
}

/**
 * Slim horizontal bar that pulses with CPU pressure.
 *
 * Decorative only — it gives the overview row a sense that the cluster is
 * "breathing" without adding another numeric metric.
 */
function CpuHeartbeat({ cpuPct }: { cpuPct: number }) {
  const pct = Math.min(100, Math.max(0, cpuPct));
  return (
    <div className="relative h-1 w-full rounded-full overflow-hidden bg-white/[0.04]">
      <motion.div
        className="h-full bg-gradient-to-r from-accent-cyan via-accent-violet to-accent-green"
        animate={{ width: `${pct}%` }}
        transition={{ duration: 0.6, ease: "easeOut" }}
      />
    </div>
  );
}
