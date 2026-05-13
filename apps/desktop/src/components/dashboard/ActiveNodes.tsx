/**
 * @fileoverview Active node inventory for the local Sangam mesh.
 *
 * The panel displays this device with live metrics first, followed by each
 * discovered peer with advertised hardware specs and freshness-based status.
 */
import { motion, AnimatePresence } from "framer-motion";
import { Cpu, MemoryStick, Server, Signal, Wifi } from "lucide-react";
import { Card, CardHeader, CardTitle } from "../ui/Card";
import { Badge } from "../ui/Badge";
import {
  useMetrics,
  useNodeInfo,
  usePeers,
} from "../../hooks/use-runtime-data";
import { formatPct, formatRelativeTime, shortId } from "../../lib/format";
import { cn } from "../../lib/cn";
import type { Peer } from "../../lib/types";

/**
 * Section 3 — Active Nodes Panel.
 *
 * Shows this device first (with live metrics), then every discovered peer as a
 * card.  Cards animate in/out as peers come and go.
 */
export function ActiveNodes() {
  const { data: info } = useNodeInfo();
  const { data: metrics } = useMetrics();
  const { data: peers } = usePeers();

  return (
    <Card>
      <CardHeader>
        <div>
          <CardTitle className="flex items-center gap-2">
            <Server className="w-3.5 h-3.5 text-accent-violet" />
            Active Nodes
          </CardTitle>
          <p className="text-xs text-ink-muted mt-1">
            {peers.length === 0
              ? "This device only"
              : `${peers.length + 1} nodes including self`}
          </p>
        </div>
        <Badge tone={peers.length > 0 ? "cyan" : "neutral"}>
          {peers.length + 1} online
        </Badge>
      </CardHeader>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
        {/* Self card — always first, always present */}
        <NodeCard
          isSelf
          name="This device"
          subtitle={`${info.local_ip}:${info.port}`}
          cpu={metrics?.cpu_pct ?? 0}
          ramPct={
            metrics && metrics.ram_total_mb > 0
              ? (metrics.ram_used_mb / metrics.ram_total_mb) * 100
              : 0
          }
          latencyMs={0}
          contribution={100} // we host the runtime; baseline 100%
          status={info.running ? "active" : "idle"}
        />

        <AnimatePresence>
          {peers.map((p) => (
            <PeerNodeCard key={p.id} peer={p} totalPeers={peers.length} />
          ))}
        </AnimatePresence>
      </div>
    </Card>
  );
}

/**
 * Renders a remote peer using advertised specs rather than live telemetry.
 *
 * Sangam does not yet stream CPU/RAM usage from peers, so this card follows the
 * honest UI principle: show mDNS-advertised capacity and freshness instead of
 * fabricating live utilisation values.
 */
function PeerNodeCard({
  peer,
  totalPeers,
}: {
  peer: Peer;
  totalPeers: number;
}) {
  const ageMs = Date.now() - peer.last_seen_ms;
  const status: NodeStatus =
    ageMs < 5000 ? "active" : ageMs < 15000 ? "busy" : "stale";

  // Even contribution share for now — once the scheduler dispatches by
  // capacity, this can read from the task tracker.
  const contribution = totalPeers > 0 ? 100 / (totalPeers + 1) : 0;

  return (
    <NodeCard
      name={peer.name.replace("._sangam._udp.local.", "")}
      subtitle={peer.addr}
      // CPU/RAM live usage is unknown from this side; show advertised specs.
      cpuLabel={peer.cpu_threads ? `${peer.cpu_threads} threads` : "—"}
      ramLabel={peer.ram_gib ? `${peer.ram_gib} GB` : "—"}
      latencyMs={ageMs}
      contribution={contribution}
      status={status}
      shortLabel={shortId(peer.id)}
    />
  );
}

type NodeStatus = "active" | "idle" | "busy" | "stale";

const statusMeta: Record<
  NodeStatus,
  { label: string; tone: "green" | "amber" | "red" | "neutral" }
> = {
  active: { label: "Active", tone: "green" },
  idle: { label: "Idle", tone: "neutral" },
  busy: { label: "Busy", tone: "amber" },
  stale: { label: "Stale", tone: "red" },
};

// Shared node card. Self uses numeric `cpu`/`ramPct` live metrics; peers use
// `cpuLabel`/`ramLabel` strings for advertised capacity until remote telemetry exists.
function NodeCard({
  name,
  subtitle,
  cpu,
  ramPct,
  cpuLabel,
  ramLabel,
  latencyMs,
  contribution,
  status,
  isSelf,
  shortLabel,
}: {
  name: string;
  subtitle: string;
  cpu?: number;
  ramPct?: number;
  cpuLabel?: string;
  ramLabel?: string;
  latencyMs: number;
  contribution: number;
  status: NodeStatus;
  isSelf?: boolean;
  shortLabel?: string;
}) {
  const meta = statusMeta[status];
  return (
    <motion.div
      layout
      initial={{ opacity: 0, scale: 0.96 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.96 }}
      transition={{ duration: 0.2 }}
      whileHover={{ y: -2 }}
      className={cn(
        "relative rounded-xl p-4 group",
        "bg-white/[0.02] border border-white/[0.06]",
        "hover:bg-white/[0.04] hover:border-white/[0.12]",
        "transition-colors",
        isSelf && "ring-1 ring-accent-cyan/30 bg-accent-cyan/[0.03]",
      )}
    >
      {/* Activity pulse pinned to top-right */}
      <span
        className={cn(
          "absolute top-3 right-3 w-1.5 h-1.5 rounded-full",
          status === "active" && "bg-accent-green",
          status === "busy" && "bg-accent-amber",
          status === "stale" && "bg-accent-red",
          status === "idle" && "bg-ink-dim",
        )}
      >
        {status === "active" && (
          <span className="absolute inset-0 rounded-full bg-accent-green animate-ping-soft" />
        )}
      </span>

      <div className="flex items-start gap-2 mb-3">
        <div className="p-1.5 rounded-md bg-white/[0.04] border border-white/[0.06]">
          <Server
            className={cn(
              "w-3.5 h-3.5",
              isSelf ? "text-accent-cyan" : "text-ink-muted",
            )}
          />
        </div>
        <div className="min-w-0 flex-1">
          <div className="text-sm font-medium text-ink truncate">{name}</div>
          <div className="font-mono text-[10px] text-ink-dim truncate">
            {subtitle}
          </div>
        </div>
      </div>

      {/* Metric rows */}
      <div className="space-y-2 mb-3">
        <MetricRow
          icon={<Cpu className="w-3 h-3" />}
          label="CPU"
          value={cpuLabel ?? (cpu !== undefined ? formatPct(cpu, 0) : "—")}
          progress={cpu}
          tone="cyan"
        />
        <MetricRow
          icon={<MemoryStick className="w-3 h-3" />}
          label="RAM"
          value={
            ramLabel ?? (ramPct !== undefined ? formatPct(ramPct, 0) : "—")
          }
          progress={ramPct}
          tone="violet"
        />
      </div>

      {/* Footer chips */}
      <div className="flex items-center justify-between pt-3 border-t border-white/[0.04]">
        <Badge tone={meta.tone}>{meta.label}</Badge>
        <div className="flex items-center gap-3 text-[10px] text-ink-dim">
          {!isSelf && (
            <span className="flex items-center gap-1">
              <Signal className="w-2.5 h-2.5" />
              {formatRelativeTime(Date.now() - latencyMs)}
            </span>
          )}
          <span className="flex items-center gap-1">
            <Wifi className="w-2.5 h-2.5" />
            {contribution.toFixed(0)}%
          </span>
        </div>
      </div>

      {shortLabel && (
        <div className="absolute bottom-1.5 right-2 mono-tag text-[9px]">
          {shortLabel}
        </div>
      )}
    </motion.div>
  );
}

// Metric row with an optional animated progress bar. If `progress` is omitted
// (peer spec mode), only the textual value is rendered.
function MetricRow({
  icon,
  label,
  value,
  progress,
  tone,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
  progress?: number;
  tone: "cyan" | "violet";
}) {
  const bar =
    tone === "cyan"
      ? "from-accent-cyan/40 to-accent-cyan"
      : "from-accent-violet/40 to-accent-violet";
  const pct = Math.min(100, Math.max(0, progress ?? 0));
  return (
    <div>
      <div className="flex items-center justify-between text-[11px] mb-1">
        <span className="flex items-center gap-1.5 text-ink-muted">
          {icon}
          {label}
        </span>
        <span className="font-mono text-ink">{value}</span>
      </div>
      <div className="h-1 rounded-full bg-white/[0.04] overflow-hidden">
        {/* Animated width keeps rapidly changing CPU/RAM values smooth. */}
        {progress !== undefined && (
          <motion.div
            className={cn("h-full rounded-full bg-gradient-to-r", bar)}
            animate={{ width: `${pct}%` }}
            transition={{ duration: 0.6, ease: "easeOut" }}
          />
        )}
      </div>
    </div>
  );
}
