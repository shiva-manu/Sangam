/**
 * @fileoverview Live structured runtime console for Sangam events.
 *
 * Displays a tailing log stream with client-side level filters, source tags,
 * level colouring, and auto-scroll behaviour that respects user scrollback.
 */
import { useEffect, useMemo, useRef, useState } from "react";
import { motion } from "framer-motion";
import { ChevronDown, Filter, Terminal } from "lucide-react";
import { Card, CardHeader, CardTitle } from "../ui/Card";
import { Badge } from "../ui/Badge";
import { useLogs } from "../../hooks/use-runtime-data";
import { formatTime } from "../../lib/format";
import { cn } from "../../lib/cn";
import type { LogLevel } from "../../lib/types";

/**
 * Section 5 — Distributed Runtime Console.
 *
 * Live, structured tail of runtime events.  Warp-style: monospace rows,
 * level colouring, source tags, and bottom-stick auto-scroll while the user
 * remains at the end of the stream.
 */
const LEVEL_FILTERS: { value: LogLevel | "all"; label: string }[] = [
  { value: "all", label: "All" },
  { value: "info", label: "Info" },
  { value: "warn", label: "Warn" },
  { value: "error", label: "Error" },
];

export function RuntimeConsole() {
  // `useLogs` owns the logsRef ref-mirror that keeps the live Tauri listener
  // appending to fresh state instead of a stale closure.
  const logs = useLogs();
  const [filter, setFilter] = useState<LogLevel | "all">("all");
  const [autoScroll, setAutoScroll] = useState(true);
  const scrollRef = useRef<HTMLDivElement | null>(null);

  const filtered = useMemo(
    () => (filter === "all" ? logs : logs.filter((l) => l.level === filter)),
    [logs, filter],
  );

  // Auto-scroll to bottom whenever new entries arrive AND the user is
  // still pinned to the bottom. If they've scrolled up, leave them be.
  useEffect(() => {
    if (!autoScroll) return;
    const el = scrollRef.current;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
  }, [filtered, autoScroll]);

  const handleScroll = () => {
    const el = scrollRef.current;
    if (!el) return;
    // Auto-scroll detection: if the remaining scroll distance is <16 px,
    // consider the user pinned to the bottom and keep tailing new entries.
    const atBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 16;
    setAutoScroll(atBottom);
  };

  const jumpToBottom = () => {
    const el = scrollRef.current;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
    setAutoScroll(true);
  };

  return (
    <Card className="h-[420px] flex flex-col">
      <CardHeader>
        <div>
          <CardTitle className="flex items-center gap-2">
            <Terminal className="w-3.5 h-3.5 text-accent-cyan" />
            Runtime Console
          </CardTitle>
          <p className="text-xs text-ink-muted mt-1">
            Structured event stream from discovery, networking, tasks
          </p>
        </div>

        <div className="flex items-center gap-2">
          <div className="flex items-center gap-0.5 p-0.5 rounded-md bg-white/[0.03] border border-white/[0.04]">
            <Filter className="w-3 h-3 text-ink-dim ml-1.5" />
            {LEVEL_FILTERS.map((f) => (
              <button
                key={f.value}
                onClick={() => setFilter(f.value)}
                className={cn(
                  "px-2 py-0.5 text-[10px] font-medium uppercase tracking-wider rounded",
                  "transition-colors",
                  filter === f.value
                    ? "bg-white/[0.06] text-ink"
                    : "text-ink-dim hover:text-ink-muted",
                )}
              >
                {f.label}
              </button>
            ))}
          </div>
        </div>
      </CardHeader>

      <div className="relative flex-1 min-h-0 -mx-2">
        <div
          ref={scrollRef}
          onScroll={handleScroll}
          className={cn(
            "absolute inset-0 px-3 overflow-y-auto",
            "bg-black/40 rounded-lg border border-white/[0.04]",
            "font-mono text-[11px] leading-relaxed",
          )}
        >
          {filtered.length === 0 ? (
            <div className="h-full flex items-center justify-center text-ink-dim text-xs italic">
              Waiting for runtime events…
            </div>
          ) : (
            <div className="py-2">
              {filtered.map((entry, i) => (
                <LogLine key={`${entry.timestamp_ms}-${i}`} entry={entry} />
              ))}
            </div>
          )}
        </div>

        {/* "Jump to bottom" pill when user is scrolled up */}
        {!autoScroll && (
          <motion.button
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 6 }}
            onClick={jumpToBottom}
            className={cn(
              "absolute bottom-3 right-3 flex items-center gap-1.5",
              "px-2.5 py-1 rounded-full text-[10px] font-medium",
              "bg-accent-cyan/15 text-accent-cyan border border-accent-cyan/30",
              "shadow-lg hover:bg-accent-cyan/20",
            )}
          >
            <ChevronDown className="w-3 h-3" />
            Tail
          </motion.button>
        )}
      </div>

      {/* Footer: log count */}
      <div className="flex items-center justify-between mt-3 pt-3 border-t border-white/[0.04]">
        <span className="text-[10px] text-ink-dim">
          {filtered.length} / {logs.length} entries
        </span>
        <div className="flex items-center gap-2">
          <Badge tone={autoScroll ? "cyan" : "neutral"}>
            {autoScroll ? "Tailing" : "Paused"}
          </Badge>
        </div>
      </div>
    </Card>
  );
}

// Level metadata drives log-line colouring: debug is muted, info is cyan,
// warn is amber, and error is red to match the rest of the dashboard palette.
const levelMeta: Record<
  LogLevel,
  { dot: string; label: string; chip: "neutral" | "cyan" | "amber" | "red" }
> = {
  debug: { dot: "bg-ink-dim", label: "DBG", chip: "neutral" },
  info: { dot: "bg-accent-cyan", label: "INF", chip: "cyan" },
  warn: { dot: "bg-accent-amber", label: "WRN", chip: "amber" },
  error: { dot: "bg-accent-red", label: "ERR", chip: "red" },
};

// Individual console row: timestamp, severity chip, source tag, then message.
function LogLine({
  entry,
}: {
  entry: {
    timestamp_ms: number;
    level: LogLevel;
    source: string;
    message: string;
  };
}) {
  const meta = levelMeta[entry.level];
  return (
    <div className="group flex items-start gap-2 py-0.5 hover:bg-white/[0.02] -mx-3 px-3 rounded">
      <span className="text-ink-dim tabular-nums shrink-0">
        {formatTime(entry.timestamp_ms)}
      </span>
      <span
        className={cn(
          "shrink-0 text-[10px] font-semibold tracking-wider",
          entry.level === "info" && "text-accent-cyan",
          entry.level === "warn" && "text-accent-amber",
          entry.level === "error" && "text-accent-red",
          entry.level === "debug" && "text-ink-dim",
        )}
      >
        {meta.label}
      </span>
      <span className="shrink-0 text-ink-muted">[{entry.source}]</span>
      <span className="text-ink min-w-0 break-words">{entry.message}</span>
    </div>
  );
}
