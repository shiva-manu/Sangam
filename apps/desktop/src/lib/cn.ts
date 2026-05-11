import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/// Tailwind-aware className combiner. Standard pattern: `clsx` builds the
/// list, `twMerge` resolves conflicts (e.g. `p-2 p-4` → `p-4`).
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
