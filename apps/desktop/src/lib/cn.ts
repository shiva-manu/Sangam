/**
 * @fileoverview Tailwind-aware className combiner utility.
 *
 * Exports the `cn` helper — the standard "clsx + tailwind-merge" pattern
 * used across the entire UI layer.  `clsx` handles conditionals, arrays, and
 * object maps; `twMerge` resolves Tailwind utility conflicts so that the last
 * rule wins (e.g. `p-2 p-4` collapses to `p-4`).
 */
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Merges and deduplicates Tailwind CSS class names.
 *
 * Accepts any combination of strings, arrays, objects, or falsy values
 * (handled by `clsx`), then resolves Tailwind-specific conflicts so that
 * later classes win over earlier ones in the same utility group.
 *
 * @param inputs - One or more class values to combine.  Falsy values are
 *   ignored; objects are treated as `{ [className]: boolean }` condition maps.
 * @returns A single deduplicated class string ready to pass to `className`.
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
