/**
 * @fileoverview Persistent top bar for runtime controls and cluster pulse.
 *
 * Mounted once by `AppShell`, this component shows mesh status, live resource
 * stats, search, and the runtime Start/Stop control across every page.
 */
import { motion } from "framer-motion";
import { Cpu, MemoryStick, Search, Wifi, Power } from "lucide-react";
import { Button } from "../ui/Button";
import { Input } from "../ui/Input";
import {
  useMetrics,
  useNodeInfo,
  usePeers,
} from "../../hooks/use-runtime-data";
import { api } from "../../lib/tauri";
import { formatPct, formatMib } from "../../lib/format";
import { cn } from "../../lib/cn";

/**
 * Renders cluster status, live CPU/RAM stats, peer count, search, and runtime
 * control actions in the persistent application header.
 */
export function TopBar() {
  const { data: info, refresh: refreshInfo } = useNodeInfo();
  const { data: peers } = usePeers();
  const { data: metrics } = useMetrics();

  const onlineCount = peers.length;
  const running = info.running;

  const cpu = metrics?.cpu_pct ?? 0;
  const ram =
    metrics && metrics.ram_total_mb > 0
      ? (metrics.ram_used_mb / metrics.ram_total_mb) * 100
      : 0;

  const handleToggle = async () => {
    // Optimistic refresh pattern: mutate runtime state, then immediately ask
    // the node-info hook to refetch so the status pill updates without waiting
    // for the next 1 s polling tick.
    try {
      if (running) {
        await api.stopRuntime();
      } else {
        await api.startRuntime();
      }
      void refreshInfo();
    } catch (e) {
      // Failures surface via the existing error state inside hooks; don't
      // throw — we'd kill the app shell.
      console.error("runtime toggle failed", e);
    }
  };

  return (
    <header className="relative z-10 h-14 shrink-0 border-b border-white/[0.04] bg-bg-surface/30 backdrop-blur-xl">
      <div className="h-full px-5 flex items-center gap-4">
        {/* Cluster status pill */}
        <div className="flex items-center gap-2.5">
          <div
            className={cn(
              "status-dot",
              running ? "text-accent-green" : "text-ink-dim",
            )}
            style={{ backgroundColor: "currentColor" }}
          />
          <div className="flex items-baseline gap-2">
            <span className="text-sm font-medium text-ink">
              {running ? "Mesh online" : "Idle"}
            </span>
            <span className="mono-tag">
              {info.local_ip}:{info.port}
            </span>
          </div>
        </div>

        <Divider />

        {/* Connected peers */}
        <Stat
          icon={<Wifi className="w-3.5 h-3.5" />}
          label="Peers"
          value={String(onlineCount)}
          tone={onlineCount > 0 ? "ok" : "neutral"}
        />

        <Divider />

        {/* CPU */}
        <Stat
          icon={<Cpu className="w-3.5 h-3.5" />}
          label="CPU"
          value={formatPct(cpu, 0)}
          tone={cpu > 85 ? "warn" : "neutral"}
        />

        {/* RAM */}
        <Stat
          icon={<MemoryStick className="w-3.5 h-3.5" />}
          label="RAM"
          value={
            metrics
              ? `${formatMib(metrics.ram_used_mb)} (${formatPct(ram, 0)})`
              : "—"
          }
          tone={ram > 90 ? "warn" : "neutral"}
        />

        <div className="flex-1" />

        {/* Search */}
        <div className="relative w-72">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-ink-dim" />
          <Input
            placeholder="Search nodes, tasks, logs…"
            className="pl-8 h-8 text-xs"
          />
        </div>

        {/* Start / Stop */}
        <Button
          size="sm"
          variant={running ? "danger" : "primary"}
          onClick={handleToggle}
        >
          <Power className="w-3.5 h-3.5" />
          {running ? "Stop" : "Start"}
        </Button>
      </div>

      {/* Pure decoration: a subtle bottom-edge shimmer while running gives the
          persistent bar a pulse without conveying additional data. */}
      {running && (
        <motion.div
          className="absolute bottom-0 left-0 right-0 h-px overflow-hidden"
          aria-hidden
        >
          <motion.div
            className="h-full w-1/3 bg-gradient-to-r from-transparent via-accent-cyan/60 to-transparent"
            animate={{ x: ["-100%", "300%"] }}
            transition={{ duration: 3, repeat: Infinity, ease: "linear" }}
          />
        </motion.div>
      )}
    </header>
  );
}

// Small vertical rule used to group status clusters without extra markup noise.
function Divider() {
  return <div className="w-px h-5 bg-white/[0.06]" />;
}

// Compact metric helper used for peer count, CPU, and RAM in the constrained
// top-bar layout.
function Stat({
  icon,
  label,
  value,
  tone,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
  tone: "neutral" | "ok" | "warn";
}) {
  const toneClass =
    tone === "ok"
      ? "text-accent-green"
      : tone === "warn"
        ? "text-accent-amber"
        : "text-ink-muted";
  return (
    <div className="flex items-center gap-2">
      <span className={cn("flex items-center", toneClass)}>{icon}</span>
      <div className="flex flex-col leading-tight">
        <span className="text-[10px] uppercase tracking-wider text-ink-dim">
          {label}
        </span>
        <span className="text-xs font-medium text-ink">{value}</span>
      </div>
    </div>
  );
}
