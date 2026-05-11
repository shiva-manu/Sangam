// Display formatters used across the dashboard. Centralised so we can
// tweak rounding/units in one place and have everything follow.

export function formatPct(n: number, digits = 1): string {
  if (!Number.isFinite(n)) return "—";
  return `${n.toFixed(digits)}%`;
}

export function formatMib(mb: number): string {
  if (!Number.isFinite(mb) || mb <= 0) return "0 MB";
  if (mb < 1024) return `${mb.toFixed(0)} MB`;
  return `${(mb / 1024).toFixed(2)} GB`;
}

export function formatKbps(kbps: number): string {
  if (!Number.isFinite(kbps) || kbps < 0.05) return "0 KB/s";
  if (kbps < 1024) return `${kbps.toFixed(1)} KB/s`;
  return `${(kbps / 1024).toFixed(2)} MB/s`;
}

export function formatRelativeTime(ts: number, now = Date.now()): string {
  const diff = Math.max(0, now - ts);
  if (diff < 1000) return "just now";
  const s = Math.floor(diff / 1000);
  if (s < 60) return `${s}s ago`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m ago`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h}h ago`;
  const d = Math.floor(h / 24);
  return `${d}d ago`;
}

export function formatTime(ts: number): string {
  const d = new Date(ts);
  return d.toLocaleTimeString(undefined, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });
}

/// Short, identifier-shaped slice — first 8 chars, used for node IDs,
/// task IDs, and anywhere a full UUID would be too noisy.
export function shortId(id: string): string {
  return id.slice(0, 8);
}
