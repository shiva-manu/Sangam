/**
 * @fileoverview Generic polling hook for React components.
 *
 * `usePolled` centralises the dashboard's "fetch immediately, then refresh on
 * an interval" behaviour.  It is intentionally UI-safe: failures become
 * component state instead of thrown errors, so one unhealthy data source does
 * not unmount the whole panel tree.
 */
import { useEffect, useRef, useState } from "react";

/**
 * Polls an asynchronous function on mount and at a fixed interval.
 *
 * The hook calls `fn` once immediately after mount, then again every
 * `intervalMs` milliseconds.  Errors are stored in state rather than thrown,
 * allowing callers to render a muted or retry state without crashing React.
 *
 * @typeParam T - The resolved data type returned by `fn`.
 * @param fn - Async producer that fetches the latest value.  The latest
 *   function reference is always used, even if the interval was created by an
 *   earlier render.
 * @param intervalMs - Polling interval in milliseconds.
 * @param initial - Initial value returned before the first successful poll.
 * @returns An object containing:
 *   - `data`: the latest successfully fetched value.
 *   - `error`: the last error string, or `null` after a successful refresh.
 *   - `refresh`: an imperative refresh function callers can invoke on demand.
 */
export function usePolled<T>(
  fn: () => Promise<T>,
  intervalMs: number,
  initial: T,
): { data: T; error: string | null; refresh: () => Promise<void> } {
  const [data, setData] = useState<T>(initial);
  const [error, setError] = useState<string | null>(null);
  // Ref-mirror the fetcher to avoid stale closures inside the interval.
  // This lets callers pass a new `fn` without tearing down/recreating the timer.
  const fnRef = useRef(fn);
  fnRef.current = fn;

  const refresh = async () => {
    try {
      const next = await fnRef.current();
      setData(next);
      setError(null);
    } catch (e) {
      setError(String(e));
    }
  };

  useEffect(() => {
    // Guards the initial async fetch so it cannot set state after unmount.
    let cancelled = false;
    void (async () => {
      try {
        const next = await fnRef.current();
        if (!cancelled) {
          setData(next);
          setError(null);
        }
      } catch (e) {
        if (!cancelled) setError(String(e));
      }
    })();
    const id = setInterval(() => {
      void refresh();
    }, intervalMs);
    return () => {
      cancelled = true;
      clearInterval(id);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [intervalMs]);

  return { data, error, refresh };
}
