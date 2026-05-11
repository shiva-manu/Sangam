import * as SwitchPrimitive from "@radix-ui/react-switch";
import { forwardRef, type ComponentPropsWithoutRef } from "react";
import { cn } from "../../lib/cn";

export const Switch = forwardRef<
  HTMLButtonElement,
  ComponentPropsWithoutRef<typeof SwitchPrimitive.Root>
>(({ className, ...props }, ref) => (
  <SwitchPrimitive.Root
    ref={ref}
    className={cn(
      "peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center",
      "rounded-full border border-white/10",
      "bg-white/[0.04] transition-colors",
      "data-[state=checked]:bg-accent-cyan/30 data-[state=checked]:border-accent-cyan/50",
      "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-cyan/40",
      "disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
  >
    <SwitchPrimitive.Thumb
      className={cn(
        "pointer-events-none block h-4 w-4 rounded-full bg-ink shadow-lg",
        "transition-transform duration-150",
        "translate-x-0.5 data-[state=checked]:translate-x-[18px]",
        "data-[state=checked]:bg-accent-cyan",
      )}
    />
  </SwitchPrimitive.Root>
));
Switch.displayName = "Switch";
