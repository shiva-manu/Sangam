/**
 * @fileoverview Styled text input primitive used by the top bar search.
 *
 * Keeps form controls visually aligned with the rest of the glass UI system.
 */
import { forwardRef, type InputHTMLAttributes } from "react";
import { cn } from "../../lib/cn";

/**
 * Native input with Sangam surface styling.
 *
 * The focus-visible cyan ring matches other interactive controls, giving
 * keyboard users a consistent visual target across buttons, switches, sliders,
 * and search fields.
 */
export const Input = forwardRef<
  HTMLInputElement,
  InputHTMLAttributes<HTMLInputElement>
>(({ className, type = "text", ...props }, ref) => (
  <input
    ref={ref}
    type={type}
    className={cn(
      "flex h-9 w-full rounded-lg",
      "bg-white/[0.03] border border-white/10",
      "px-3 py-1 text-sm text-ink placeholder:text-ink-dim",
      "transition-colors",
      "focus-visible:outline-none focus-visible:border-accent-cyan/60",
      "focus-visible:ring-2 focus-visible:ring-accent-cyan/20",
      "disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
  />
));
Input.displayName = "Input";
