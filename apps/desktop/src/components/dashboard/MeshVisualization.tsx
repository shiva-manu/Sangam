import { useMemo } from "react";
import { motion } from "framer-motion";
import { Activity, Wifi } from "lucide-react";
import { Card, CardHeader, CardTitle } from "../ui/Card";
import { Badge } from "../ui/Badge";
import { useNodeInfo, usePeers } from "../../hooks/use-runtime-data";
import { shortId } from "../../lib/format";

/// Section 2 — Live Node Mesh Visualization.
///
/// Center node = this device. Peers orbit around it in a circle. Lines
/// show the active links; an animated dot travels along each line to
/// suggest packet flow. Node colour reflects freshness:
///   * green  — seen <5s ago (healthy)
///   * amber  — seen <15s ago (busy / late ping)
///   * red    — seen ≥15s ago (likely gone)
///
/// SVG-based so it scales crisply and animations are GPU-accelerated.
export function MeshVisualization() {
  const { data: info } = useNodeInfo();
  const { data: peers } = usePeers();

  // Layout: position each peer evenly on a circle around the centre.
  // Stable order (by id) so the same peer always lands in the same slot
  // across re-renders — avoids "flicker reshuffling" as peers update.
  const layout = useMemo(() => {
    const sorted = [...peers].sort((a, b) => a.id.localeCompare(b.id));
    const N = sorted.length;
    const radius = 38; // % of viewBox half — leaves room for labels
    return sorted.map((p, i) => {
      // Start at the top (-90°), go clockwise. If only one peer, push
      // slightly off-axis so the connection line doesn't sit dead-vertical.
      const offset = N === 1 ? -75 : -90;
      const angle = ((360 / Math.max(N, 1)) * i + offset) * (Math.PI / 180);
      return {
        peer: p,
        x: 50 + radius * Math.cos(angle),
        y: 50 + radius * Math.sin(angle),
      };
    });
  }, [peers]);

  return (
    <Card className="relative h-[420px] overflow-hidden">
      <CardHeader>
        <div>
          <CardTitle className="flex items-center gap-2">
            <Wifi className="w-3.5 h-3.5 text-accent-cyan" />
            Live Mesh Topology
          </CardTitle>
          <p className="text-xs text-ink-muted mt-1">
            Self-organising peer graph over local mDNS
          </p>
        </div>
        <Badge tone={info.running ? "green" : "neutral"}>
          {info.running ? "Broadcasting" : "Idle"}
        </Badge>
      </CardHeader>

      <div className="absolute inset-0 grid-backdrop opacity-50 pointer-events-none" />

      {/* SVG canvas. Use a 100×100 viewBox + percentage coords so layout
          math is unitless and the whole thing scales with the container. */}
      <svg
        viewBox="0 0 100 100"
        className="absolute inset-0 w-full h-full"
        preserveAspectRatio="xMidYMid meet"
      >
        <defs>
          {/* Soft gradient stroke for connection lines */}
          <linearGradient id="link-gradient" x1="0" x2="1" y1="0" y2="0">
            <stop offset="0%" stopColor="rgb(56 189 248)" stopOpacity="0.0" />
            <stop offset="50%" stopColor="rgb(56 189 248)" stopOpacity="0.5" />
            <stop offset="100%" stopColor="rgb(167 139 250)" stopOpacity="0.0" />
          </linearGradient>
          {/* Glow filter for active nodes */}
          <filter id="node-glow" x="-50%" y="-50%" width="200%" height="200%">
            <feGaussianBlur stdDeviation="0.8" />
          </filter>
        </defs>

        {/* Connection lines + animated packet dots */}
        {layout.map(({ peer, x, y }) => {
          const fresh = freshness(peer.last_seen_ms);
          return (
            <g key={peer.id}>
              <line
                x1={50}
                y1={50}
                x2={x}
                y2={y}
                stroke="url(#link-gradient)"
                strokeWidth={0.25}
              />
              {/* Animated packet: a small circle that travels along the link.
                  Two waves offset by 50% so flow looks continuous. */}
              <PacketDot x1={50} y1={50} x2={x} y2={y} delay={0} />
              <PacketDot x1={x} y1={y} x2={50} y2={50} delay={1.2} />
              {/* Node */}
              <circle
                cx={x}
                cy={y}
                r={2.5}
                fill={fresh.fill}
                filter="url(#node-glow)"
              />
              <circle
                cx={x}
                cy={y}
                r={1.6}
                fill={fresh.fill}
                stroke="rgb(255 255 255 / 0.3)"
                strokeWidth={0.15}
              />
              {/* Subtle outer ring pulse */}
              <circle
                cx={x}
                cy={y}
                r={2.5}
                fill="none"
                stroke={fresh.fill}
                strokeWidth={0.2}
                opacity={0.4}
              >
                <animate
                  attributeName="r"
                  values="2.5;4.5;2.5"
                  dur="2.6s"
                  repeatCount="indefinite"
                />
                <animate
                  attributeName="opacity"
                  values="0.4;0;0.4"
                  dur="2.6s"
                  repeatCount="indefinite"
                />
              </circle>
            </g>
          );
        })}

        {/* Centre node — this device. Slightly larger + cyan accent. */}
        <g>
          <circle
            cx={50}
            cy={50}
            r={4}
            fill="rgb(56 189 248 / 0.25)"
            filter="url(#node-glow)"
          />
          <circle
            cx={50}
            cy={50}
            r={2.4}
            fill="rgb(56 189 248)"
            stroke="rgb(255 255 255 / 0.5)"
            strokeWidth={0.2}
          />
        </g>
      </svg>

      {/* HTML overlay labels — anchored over the SVG coords using % positions.
          Done with absolute-positioned divs so we get crisp text and easy
          truncation, not SVG <text>. */}
      <div className="absolute inset-0 pointer-events-none">
        {/* Self label */}
        <NodeLabel x={50} y={50} title="this node" subtitle={info.local_ip} isSelf />
        {layout.map(({ peer, x, y }) => (
          <NodeLabel
            key={peer.id}
            x={x}
            y={y}
            title={shortId(peer.id)}
            subtitle={peer.addr}
          />
        ))}
      </div>

      {/* Empty state */}
      {peers.length === 0 && (
        <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
          <div className="mt-12 text-center">
            <Activity className="w-6 h-6 text-accent-cyan/40 mx-auto mb-2 animate-pulse-slow" />
            <div className="text-xs text-ink-muted">
              No peers yet — waiting for nearby Sangam nodes to join…
            </div>
          </div>
        </div>
      )}

      {/* Legend */}
      <div className="absolute bottom-3 right-3 flex items-center gap-3 text-[10px] text-ink-dim">
        <LegendDot tone="green" label="Healthy" />
        <LegendDot tone="amber" label="Busy" />
        <LegendDot tone="red" label="Stale" />
      </div>
    </Card>
  );
}

function freshness(lastSeenMs: number): { fill: string; tone: string } {
  const age = Date.now() - lastSeenMs;
  if (age < 5000) return { fill: "rgb(52 211 153)", tone: "green" };
  if (age < 15000) return { fill: "rgb(251 191 36)", tone: "amber" };
  return { fill: "rgb(248 113 113)", tone: "red" };
}

function PacketDot({
  x1,
  y1,
  x2,
  y2,
  delay,
}: {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  delay: number;
}) {
  return (
    <circle r="0.45" fill="rgb(56 189 248)">
      <animate
        attributeName="cx"
        values={`${x1};${x2}`}
        dur="2.4s"
        begin={`${delay}s`}
        repeatCount="indefinite"
      />
      <animate
        attributeName="cy"
        values={`${y1};${y2}`}
        dur="2.4s"
        begin={`${delay}s`}
        repeatCount="indefinite"
      />
      <animate
        attributeName="opacity"
        values="0;1;1;0"
        dur="2.4s"
        begin={`${delay}s`}
        repeatCount="indefinite"
      />
    </circle>
  );
}

function NodeLabel({
  x,
  y,
  title,
  subtitle,
  isSelf,
}: {
  x: number;
  y: number;
  title: string;
  subtitle?: string;
  isSelf?: boolean;
}) {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.6 }}
      className="absolute -translate-x-1/2 translate-y-3 text-center"
      style={{ left: `${x}%`, top: `${y}%` }}
    >
      <div
        className={
          isSelf
            ? "text-[10px] font-medium text-accent-cyan"
            : "text-[10px] font-medium text-ink"
        }
      >
        {title}
      </div>
      {subtitle && (
        <div className="font-mono text-[9px] text-ink-dim mt-0.5">
          {subtitle}
        </div>
      )}
    </motion.div>
  );
}

function LegendDot({
  tone,
  label,
}: {
  tone: "green" | "amber" | "red";
  label: string;
}) {
  const colors = {
    green: "bg-accent-green",
    amber: "bg-accent-amber",
    red: "bg-accent-red",
  };
  return (
    <span className="flex items-center gap-1.5">
      <span className={`w-1.5 h-1.5 rounded-full ${colors[tone]}`} />
      {label}
    </span>
  );
}
