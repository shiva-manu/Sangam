import { forwardRef, type ButtonHTMLAttributes } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/cn";

const buttonStyles = cva(
  // Base — every button gets these.
  [
    "inline-flex items-center justify-center gap-2",
    "rounded-lg text-sm font-medium",
    "transition-all duration-150",
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-cyan/40",
    "disabled:opacity-40 disabled:pointer-events-none",
    "select-none whitespace-nowrap",
  ].join(" "),
  {
    variants: {
      variant: {
        // Primary: cyan-tinted glass with subtle inner glow, hovers brighter.
        primary: [
          "bg-accent-cyan/10 text-accent-cyan",
          "border border-accent-cyan/30",
          "hover:bg-accent-cyan/15 hover:border-accent-cyan/50",
          "shadow-[inset_0_1px_0_0_rgb(255_255_255_/_0.06)]",
        ].join(" "),
        // Danger: same shape, red accent.
        danger: [
          "bg-accent-red/10 text-accent-red",
          "border border-accent-red/30",
          "hover:bg-accent-red/15 hover:border-accent-red/50",
        ].join(" "),
        // Ghost: minimal, used for cancel-ish actions.
        ghost: [
          "text-ink-muted",
          "hover:bg-white/[0.04] hover:text-ink",
        ].join(" "),
        // Outline: defined edge but transparent fill.
        outline: [
          "bg-transparent text-ink",
          "border border-white/10",
          "hover:bg-white/[0.04] hover:border-white/20",
        ].join(" "),
      },
      size: {
        sm: "h-7 px-2.5 text-xs",
        md: "h-9 px-3.5",
        lg: "h-11 px-5",
        icon: "h-9 w-9 p-0",
      },
    },
    defaultVariants: { variant: "outline", size: "md" },
  },
);

export interface ButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonStyles> {}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, ...props }, ref) => (
    <button
      ref={ref}
      className={cn(buttonStyles({ variant, size }), className)}
      {...props}
    />
  ),
);
Button.displayName = "Button";
