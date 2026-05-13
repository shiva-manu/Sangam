/**
 * @fileoverview Coloured status badge primitive used across the dashboard.
 *
 * Badges provide compact, semantic labels for node health, task state,
 * runtime status, and lightweight counters.
 */
import type { HTMLAttributes } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/cn";

const badgeStyles = cva(
  [
    "inline-flex items-center gap-1.5 rounded-full",
    "px-2 py-0.5 text-[10px] font-medium tracking-wider uppercase",
    "border",
  ].join(" "),
  {
    variants: {
      tone: {
        neutral: "bg-white/[0.04] text-ink-muted border-white/10",
        cyan: "bg-accent-cyan/10 text-accent-cyan border-accent-cyan/30",
        violet:
          "bg-accent-violet/10 text-accent-violet border-accent-violet/30",
        green: "bg-accent-green/10 text-accent-green border-accent-green/30",
        amber: "bg-accent-amber/10 text-accent-amber border-accent-amber/30",
        red: "bg-accent-red/10 text-accent-red border-accent-red/30",
      },
    },
    defaultVariants: { tone: "neutral" },
  },
);

/**
 * Props for the `Badge` primitive.
 *
 * `tone` communicates semantic status using the shared accent palette:
 * - `neutral` — grey/default/idle state.
 * - `cyan` — active or informational state.
 * - `green` — healthy or successful state.
 * - `amber` — warning or degraded state.
 * - `red` — error, failed, or stale state.
 * - `violet` — miscellaneous accent where no status meaning is implied.
 */
export interface BadgeProps
  extends HTMLAttributes<HTMLSpanElement>, VariantProps<typeof badgeStyles> {}

export function Badge({ className, tone, ...props }: BadgeProps) {
  return <span className={cn(badgeStyles({ tone }), className)} {...props} />;
}
