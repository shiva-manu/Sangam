import { forwardRef, type InputHTMLAttributes } from "react";
import { cn } from "../../lib/cn";

export const Input = forwardRef<HTMLInputElement, InputHTMLAttributes<HTMLInputElement>>(
  ({ className, type = "text", ...props }, ref) => (
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
  ),
);
Input.displayName = "Input";
