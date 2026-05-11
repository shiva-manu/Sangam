import { useMemo } from "react";
import {
  Area,
  AreaChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { Activity, ArrowDown, ArrowUp, Cpu, MemoryStick } from "lucide-react";
import { Card, CardHeader, CardTitle } from "../ui/Card";
import { Badge } from "../ui/Badge";
import {
  useMetricsHistory,
  useStatusCounts,
} from "../../hooks/use-runtime-data";
import { formatKbps, formatPct } from "../../lib/format";
import type { MetricsSample } from "../../lib/types";

/// Section 6 — Cluster Resource Graphs.
///
/// Four small charts in a 2×2 grid: CPU, RAM, network throughput, and
/// task throughput. Recharts with gradient fills + cyan/violet palette.
export function ResourceGraphs() {
  const { data: samples } = useMetricsHistory();
  const { data: counts } = useStatusCounts();

  // Recharts wants numeric x-axis values; map timestamps to seconds-relative
  // to the most-recent sample so the axis reads as "Xs ago" cleanly.
  const data = useMemo(() => transform(samples), [samples]);

  const latest: MetricsSample | undefined = samples[samples.length - 1];
  const ramPct =
    latest && latest.ram_total_mb > 0
      ? (latest.ram_used_mb / latest.ram_total_mb) * 100
      : 0;

  return (
    <Card>
      <CardHeader>
        <div>
          <CardTitle className="flex items-center gap-2">
            <Activity className="w-3.5 h-3.5 text-accent-cyan" />
            Resource Pulse
          </CardTitle>
          <p className="text-xs text-ink-muted mt-1">
            Last {samples.length} samples · 1s cadence
          </p>
        </div>
        <Badge tone="cyan">live</Badge>
      </CardHeader>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
        <MiniChart
          title="CPU Usage"
          icon={<Cpu className="w-3 h-3" />}
          accent="cyan"
          current={formatPct(latest?.cpu_pct ?? 0, 0)}
          data={data}
          dataKey="cpu"
          unit="%"
          domain={[0, 100]}
        />
        <MiniChart
          title="RAM Usage"
          icon={<MemoryStick className="w-3 h-3" />}
          accent="violet"
          current={formatPct(ramPct, 0)}
          data={data}
          dataKey="ram"
          unit="%"
          domain={[0, 100]}
        />
        <MiniChart
          title="Network ↓ / ↑"
          icon={<ArrowDown className="w-3 h-3" />}
          accent="green"
          current={`${formatKbps(latest?.net_rx_kbps ?? 0)} / ${formatKbps(latest?.net_tx_kbps ?? 0)}`}
          data={data}
          dataKey="rx"
          secondKey="tx"
          unit=" KB/s"
        />
        <TaskThroughput counts={counts} />
      </div>
    </Card>
  );
}

type SamplePoint = {
  t: number;
  cpu: number;
  ram: number;
  rx: number;
  tx: number;
};

function transform(samples: MetricsSample[]): SamplePoint[] {
  if (samples.length === 0) return [];
  const last = samples[samples.length - 1].timestamp_ms;
  return samples.map((s) => ({
    t: Math.round((s.timestamp_ms - last) / 1000),
    cpu: Number(s.cpu_pct.toFixed(1)),
    ram:
      s.ram_total_mb > 0
        ? Number(((s.ram_used_mb / s.ram_total_mb) * 100).toFixed(1))
        : 0,
    rx: Number(s.net_rx_kbps.toFixed(1)),
    tx: Number(s.net_tx_kbps.toFixed(1)),
  }));
}

const accentColors: Record<
  "cyan" | "violet" | "green",
  { stroke: string; from: string; to: string }
> = {
  cyan: {
    stroke: "rgb(56 189 248)",
    from: "rgb(56 189 248 / 0.4)",
    to: "rgb(56 189 248 / 0)",
  },
  violet: {
    stroke: "rgb(167 139 250)",
    from: "rgb(167 139 250 / 0.4)",
    to: "rgb(167 139 250 / 0)",
  },
  green: {
    stroke: "rgb(52 211 153)",
    from: "rgb(52 211 153 / 0.4)",
    to: "rgb(52 211 153 / 0)",
  },
};

function MiniChart({
  title,
  icon,
  accent,
  current,
  data,
  dataKey,
  secondKey,
  unit,
  domain,
}: {
  title: string;
  icon: React.ReactNode;
  accent: keyof typeof accentColors;
  current: string;
  data: SamplePoint[];
  dataKey: keyof SamplePoint;
  secondKey?: keyof SamplePoint;
  unit?: string;
  domain?: [number, number];
}) {
  const c = accentColors[accent];
  const c2 = accentColors.violet;
  const gradId = `grad-${dataKey}`;
  const gradId2 = `grad-${dataKey}-2`;

  return (
    <div className="rounded-xl bg-white/[0.02] border border-white/[0.06] p-3">
      <div className="flex items-center justify-between mb-1">
        <div className="flex items-center gap-1.5 text-[11px] text-ink-muted">
          <span style={{ color: c.stroke }}>{icon}</span>
          {title}
        </div>
        <div className="text-xs font-mono text-ink tabular-nums">
          {current}
        </div>
      </div>
      <div className="h-24 -mx-2">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart
            data={data}
            margin={{ top: 4, right: 8, left: 8, bottom: 0 }}
          >
            <defs>
              <linearGradient id={gradId} x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor={c.from} />
                <stop offset="100%" stopColor={c.to} />
              </linearGradient>
              {secondKey && (
                <linearGradient id={gradId2} x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stopColor={c2.from} />
                  <stop offset="100%" stopColor={c2.to} />
                </linearGradient>
              )}
            </defs>
            <XAxis dataKey="t" hide />
            <YAxis hide domain={domain ?? ["auto", "auto"]} />
            <Tooltip
              content={<ChartTooltip unit={unit} />}
              cursor={{
                stroke: "rgb(255 255 255 / 0.08)",
                strokeWidth: 1,
              }}
            />
            <Area
              type="monotone"
              dataKey={dataKey as string}
              stroke={c.stroke}
              strokeWidth={1.5}
              fill={`url(#${gradId})`}
              fillOpacity={1}
              isAnimationActive
              animationDuration={400}
            />
            {secondKey && (
              <Area
                type="monotone"
                dataKey={secondKey as string}
                stroke={c2.stroke}
                strokeWidth={1.5}
                fill={`url(#${gradId2})`}
                fillOpacity={1}
                isAnimationActive
                animationDuration={400}
              />
            )}
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

/// Recharts' Tooltip content prop calls our component with `active`,
/// `payload`, etc., but the public TS type for those args has drifted
/// across versions. Declare just what we read; treat as a local contract.
type TooltipPayload = {
  color?: string;
  dataKey?: string | number;
  value?: number | string;
};
type ChartTooltipProps = {
  active?: boolean;
  payload?: TooltipPayload[];
  unit?: string;
};

function ChartTooltip({ active, payload, unit }: ChartTooltipProps) {
  if (!active || !payload || payload.length === 0) return null;
  return (
    <div className="glass !p-2 !rounded-md text-[11px] space-y-0.5">
      {payload.map((p, i) => (
        <div key={i} className="flex items-center gap-2">
          <span
            className="w-1.5 h-1.5 rounded-full"
            style={{ background: String(p.color ?? "") }}
          />
          <span className="text-ink-muted">{String(p.dataKey ?? "")}</span>
          <span className="font-mono text-ink tabular-nums">
            {String(p.value ?? "")}
            {unit}
          </span>
        </div>
      ))}
    </div>
  );
}

function TaskThroughput({
  counts,
}: {
  counts: { queued: number; running: number; completed: number; failed: number };
}) {
  const total =
    counts.queued + counts.running + counts.completed + counts.failed;
  // Show as a horizontal stacked bar with counts above.
  return (
    <div className="rounded-xl bg-white/[0.02] border border-white/[0.06] p-3">
      <div className="flex items-center justify-between mb-1">
        <div className="flex items-center gap-1.5 text-[11px] text-ink-muted">
          <ArrowUp className="w-3 h-3 text-accent-green" />
          Task Throughput
        </div>
        <div className="text-xs font-mono text-ink tabular-nums">
          {total} total
        </div>
      </div>
      <div className="h-24 flex flex-col justify-center gap-3 px-1">
        <Stacked counts={counts} />
        <div className="grid grid-cols-4 gap-2 text-[10px]">
          <Legend tone="text-ink-muted" label="queued" value={counts.queued} />
          <Legend tone="text-accent-cyan" label="running" value={counts.running} />
          <Legend tone="text-accent-green" label="done" value={counts.completed} />
          <Legend tone="text-accent-red" label="failed" value={counts.failed} />
        </div>
      </div>
    </div>
  );
}

function Stacked({
  counts,
}: {
  counts: { queued: number; running: number; completed: number; failed: number };
}) {
  const total =
    counts.queued + counts.running + counts.completed + counts.failed || 1;
  const pct = (n: number) => `${(n / total) * 100}%`;
  return (
    <div className="h-2 w-full rounded-full bg-white/[0.04] overflow-hidden flex">
      <div className="bg-ink-dim/40" style={{ width: pct(counts.queued) }} />
      <div className="bg-accent-cyan/60" style={{ width: pct(counts.running) }} />
      <div className="bg-accent-green/60" style={{ width: pct(counts.completed) }} />
      <div className="bg-accent-red/60" style={{ width: pct(counts.failed) }} />
    </div>
  );
}

function Legend({
  tone,
  label,
  value,
}: {
  tone: string;
  label: string;
  value: number;
}) {
  return (
    <div className="flex items-center justify-between">
      <span className={tone}>{label}</span>
      <span className="font-mono text-ink tabular-nums">{value}</span>
    </div>
  );
}
