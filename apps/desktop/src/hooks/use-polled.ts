import { useEffect, useRef, useState } from "react";

/// Generic polling hook: calls `fn` immediately and then every `interval` ms.
/// Cleans up on unmount and pauses cleanly across re-renders by always
/// closing over the latest `fn` via a ref.
///
/// Errors are surfaced as state, never thrown — the dashboard can show a
/// muted indicator without crashing the panel.
export function usePolled<T>(
  fn: () => Promise<T>,
  intervalMs: number,
  initial: T,
): { data: T; error: string | null; refresh: () => Promise<void> } {
  const [data, setData] = useState<T>(initial);
  const [error, setError] = useState<string | null>(null);
  // Hold the latest `fn` so the interval timer always calls the freshest
  // closure, without resetting the timer when callers pass new functions.
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
