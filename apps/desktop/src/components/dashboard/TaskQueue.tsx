import { motion, AnimatePresence } from "framer-motion";
import {
  ArrowDownLeft,
  ArrowUpRight,
  CheckCircle2,
  Clock,
  ListChecks,
  Loader2,
  XCircle,
} from "lucide-react";
import { Card, CardHeader, CardTitle } from "../ui/Card";
import { Badge } from "../ui/Badge";
import { useStatusCounts, useTasks } from "../../hooks/use-runtime-data";
import { formatRelativeTime, shortId } from "../../lib/format";
import { cn } from "../../lib/cn";
import type { TaskRecord, TaskStatus } from "../../lib/types";

/// Section 4 — Task Execution Queue.
///
/// Header carries status pills (queued / running / completed / failed).
/// Body is a scrollable list of TaskRecord rows. Each row shows
/// direction, peer, age, and a progress bar (Queued → Running → Done).
export function TaskQueue() {
  const { data: counts } = useStatusCounts();
  const { data: tasks } = useTasks();

  // Show the freshest 20 so the list stays scannable. The full set is
  // still in the tracker; this is just a viewport-friendly cap.
  const visible = tasks.slice(0, 20);

  return (
    <Card className="h-[420px] flex flex-col">
      <CardHeader>
        <div>
          <CardTitle className="flex items-center gap-2">
            <ListChecks className="w-3.5 h-3.5 text-accent-green" />
            Task Execution Queue
          </CardTitle>
          <p className="text-xs text-ink-muted mt-1">
            Live lifecycle across {tasks.length} tracked tasks
          </p>
        </div>

        {/* Status pills */}
        <div className="flex items-center gap-1.5">
          <Pill
            label="queued"
            value={counts.queued}
            icon={<Clock className="w-2.5 h-2.5" />}
            tone="neutral"
          />
          <Pill
            label="running"
            value={counts.running}
            icon={<Loader2 className="w-2.5 h-2.5 animate-spin" />}
            tone="cyan"
          />
          <Pill
            label="done"
            value={counts.completed}
            icon={<CheckCircle2 className="w-2.5 h-2.5" />}
            tone="green"
          />
          <Pill
            label="failed"
            value={counts.failed}
            icon={<XCircle className="w-2.5 h-2.5" />}
            tone="red"
          />
        </div>
      </CardHeader>

      {/* List */}
      <div className="flex-1 overflow-y-auto -mx-2 px-2">
        {visible.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-center">
            <div className="p-3 rounded-full bg-white/[0.03] border border-white/[0.06] mb-3">
              <ListChecks className="w-5 h-5 text-ink-dim" />
            </div>
            <p className="text-sm text-ink-muted">No tasks yet</p>
            <p className="text-xs text-ink-dim mt-1">
              Tasks dispatched to or received from peers will appear here
            </p>
          </div>
        ) : (
          <AnimatePresence initial={false}>
            <div className="space-y-1.5">
              {visible.map((t) => (
                <TaskRow key={t.task_id} task={t} />
              ))}
            </div>
          </AnimatePresence>
        )}
      </div>
    </Card>
  );
}

function Pill({
  label,
  value,
  icon,
  tone,
}: {
  label: string;
  value: number;
  icon: React.ReactNode;
  tone: "neutral" | "cyan" | "green" | "red";
}) {
  return (
    <Badge tone={tone} className="gap-1.5 normal-case tracking-normal">
      {icon}
      <span className="tabular-nums">{value}</span>
      <span className="text-ink-dim">{label}</span>
    </Badge>
  );
}

const statusMeta: Record<
  TaskStatus,
  {
    label: string;
    tone: "neutral" | "cyan" | "green" | "red";
    progress: number;
    icon: React.ReactNode;
  }
> = {
  queued: {
    label: "Queued",
    tone: "neutral",
    progress: 10,
    icon: <Clock className="w-3 h-3" />,
  },
  running: {
    label: "Running",
    tone: "cyan",
    progress: 60,
    icon: <Loader2 className="w-3 h-3 animate-spin" />,
  },
  completed: {
    label: "Completed",
    tone: "green",
    progress: 100,
    icon: <CheckCircle2 className="w-3 h-3" />,
  },
  failed: {
    label: "Failed",
    tone: "red",
    progress: 100,
    icon: <XCircle className="w-3 h-3" />,
  },
};

function TaskRow({ task }: { task: TaskRecord }) {
  const meta = statusMeta[task.status];
  const isOutbound = task.direction === "outbound";

  return (
    <motion.div
      layout
      initial={{ opacity: 0, x: -4 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 4 }}
      transition={{ duration: 0.18 }}
      className={cn(
        "group relative rounded-lg p-3",
        "bg-white/[0.02] border border-white/[0.04]",
        "hover:bg-white/[0.04] hover:border-white/[0.08] transition-colors",
      )}
    >
      <div className="flex items-center gap-3">
        {/* Direction icon */}
        <div
          className={cn(
            "p-1.5 rounded-md shrink-0",
            isOutbound
              ? "bg-accent-cyan/10 text-accent-cyan border border-accent-cyan/20"
              : "bg-accent-violet/10 text-accent-violet border border-accent-violet/20",
          )}
        >
          {isOutbound ? (
            <ArrowUpRight className="w-3 h-3" />
          ) : (
            <ArrowDownLeft className="w-3 h-3" />
          )}
        </div>

        {/* Title + meta */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-0.5">
            <span className="text-sm font-medium text-ink truncate">
              {task.task_type}
            </span>
            <span className="mono-tag normal-case tracking-normal text-[10px]">
              {shortId(task.task_id)}
            </span>
          </div>
          <div className="flex items-center gap-2 text-[11px] text-ink-muted">
            <span>{isOutbound ? "→ worker" : "← from"}</span>
            <span className="font-mono text-ink-dim">
              {task.peer_id ? shortId(task.peer_id) : "—"}
            </span>
            <span>·</span>
            <span>{formatRelativeTime(task.created_at_ms)}</span>
            {task.result !== undefined && (
              <>
                <span>·</span>
                <span className="text-accent-green tabular-nums">
                  = {task.result}
                </span>
              </>
            )}
          </div>
        </div>

        {/* Status chip */}
        <Badge
          tone={meta.tone}
          className="gap-1 normal-case tracking-normal shrink-0"
        >
          {meta.icon}
          {meta.label}
        </Badge>
      </div>

      {/* Progress bar */}
      <div className="mt-2 h-0.5 rounded-full overflow-hidden bg-white/[0.04]">
        <motion.div
          className={cn(
            "h-full rounded-full",
            task.status === "failed"
              ? "bg-accent-red"
              : task.status === "completed"
                ? "bg-accent-green"
                : "bg-gradient-to-r from-accent-cyan/60 to-accent-violet/60",
          )}
          animate={{ width: `${meta.progress}%` }}
          transition={{ duration: 0.4 }}
        />
      </div>

      {/* Error message, if any */}
      {task.error && (
        <div className="mt-1.5 text-[11px] text-accent-red/80 font-mono truncate">
          {task.error}
        </div>
      )}
    </motion.div>
  );
}
