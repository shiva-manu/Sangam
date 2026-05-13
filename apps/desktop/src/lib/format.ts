/**
 * @fileoverview Display formatters used across the dashboard.
 *
 * Centralising all formatters here means that tweaks to rounding,
 * unit thresholds, or localisation affect every panel at once.
 * None of these functions mutate their arguments — they are all pure.
 */

/**
 * Formats a numeric value as a percentage string.
 *
 * @param n      - The percentage value (e.g. `87.5`).
 * @param digits - Number of decimal places to display. Defaults to `1`.
 * @returns Formatted string like `"87.5%"`, or `"—"` for non-finite inputs.
 */
export function formatPct(n: number, digits = 1): string {
  if (!Number.isFinite(n)) return "—";
  return `${n.toFixed(digits)}%`;
}

/**
 * Formats a megabyte value, auto-promoting to GB when ≥ 1 024 MB.
 *
 * @param mb - Size in megabytes.
 * @returns `"0 MB"` for non-positive or non-finite input; otherwise a string
 *   such as `"512 MB"` or `"1.50 GB"`.
 */
export function formatMib(mb: number): string {
  if (!Number.isFinite(mb) || mb <= 0) return "0 MB";
  if (mb < 1024) return `${mb.toFixed(0)} MB`;
  return `${(mb / 1024).toFixed(2)} GB`;
}

/**
 * Formats a KB/s throughput value, auto-promoting to MB/s when ≥ 1 024.
 *
 * @param kbps - Network rate in kilobytes per second.
 * @returns `"0 KB/s"` for values below 0.05 or non-finite; otherwise a string
 *   such as `"128.0 KB/s"` or `"2.50 MB/s"`.
 */
export function formatKbps(kbps: number): string {
  if (!Number.isFinite(kbps) || kbps < 0.05) return "0 KB/s";
  if (kbps < 1024) return `${kbps.toFixed(1)} KB/s`;
  return `${(kbps / 1024).toFixed(2)} MB/s`;
}

/**
 * Formats a Unix-millisecond timestamp as a human-readable relative time.
 *
 * Bucketed thresholds: < 1 s → `"just now"`, < 60 s → `"Ns ago"`,
 * < 1 h → `"Nm ago"`, < 1 d → `"Nh ago"`, otherwise `"Nd ago"`.
 * Negative differences (future timestamps) are clamped to `"just now"`.
 *
 * @param ts  - Unix timestamp in milliseconds to compare against `now`.
 * @param now - Reference time in milliseconds; defaults to `Date.now()`.
 * @returns A short relative string such as `"just now"`, `"45s ago"`, or `"2d ago"`.
 */
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

/**
 * Formats a Unix-millisecond timestamp as a 24-hour wall-clock time string.
 *
 * Uses the runtime's default locale with seconds precision — suitable for
 * prefixing log lines in the Runtime Console.
 *
 * @param ts - Unix timestamp in milliseconds.
 * @returns A locale-aware time string such as `"14:03:57"`.
 */
export function formatTime(ts: number): string {
  const d = new Date(ts);
  return d.toLocaleTimeString(undefined, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });
}

/**
 * Returns the first 8 characters of an identifier string.
 *
 * Used to display node IDs, task IDs, and peer IDs wherever a full UUID
 * would be too noisy (e.g. badge labels, log lines, table cells).
 *
 * @param id - Full identifier string (typically a UUID v4).
 * @returns The leading 8-character slice, e.g. `"a1b2c3d4"`.
 */
export function shortId(id: string): string {
  return id.slice(0, 8);
}
