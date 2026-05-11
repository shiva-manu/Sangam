import * as SliderPrimitive from "@radix-ui/react-slider";
import { forwardRef, type ComponentPropsWithoutRef } from "react";
import { cn } from "../../lib/cn";

export const Slider = forwardRef<
  HTMLSpanElement,
  ComponentPropsWithoutRef<typeof SliderPrimitive.Root>
>(({ className, ...props }, ref) => (
  <SliderPrimitive.Root
    ref={ref}
    className={cn(
      "relative flex w-full touch-none select-none items-center",
      className,
    )}
    {...props}
  >
    <SliderPrimitive.Track className="relative h-1 w-full grow overflow-hidden rounded-full bg-white/[0.06]">
      <SliderPrimitive.Range className="absolute h-full bg-gradient-to-r from-accent-cyan to-accent-violet" />
    </SliderPrimitive.Track>
    <SliderPrimitive.Thumb
      className={cn(
        "block h-4 w-4 rounded-full border-2 border-accent-cyan bg-bg-base",
        "shadow-[0_0_0_4px_rgb(56_189_248_/_0.15)]",
        "transition-all hover:shadow-[0_0_0_8px_rgb(56_189_248_/_0.18)]",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-cyan/40",
        "disabled:pointer-events-none disabled:opacity-50",
      )}
    />
  </SliderPrimitive.Root>
));
Slider.displayName = "Slider";
