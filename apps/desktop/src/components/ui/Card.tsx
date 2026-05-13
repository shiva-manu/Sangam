/**
 * @fileoverview Composable card surface primitives.
 *
 * Exports the `Card` family used to build dashboard panels: `Card`,
 * `CardHeader`, `CardTitle`, `CardDescription`, and `CardContent`.
 */
import { forwardRef, type HTMLAttributes } from "react";
import { cn } from "../../lib/cn";

/**
 * Primary glass surface used for panels and grouped content.
 *
 * Compose with `glass-accent` for sections that need an extra visual hum
 * (e.g. highlighted cluster overview cards).
 */
export const Card = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("glass p-5", className)} {...props} />
  ),
);
Card.displayName = "Card";

/** Positions a card title/description block opposite optional actions. */
export const CardHeader = forwardRef<
  HTMLDivElement,
  HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex items-start justify-between gap-4 mb-4", className)}
    {...props}
  />
));
CardHeader.displayName = "CardHeader";

/** Accessible heading primitive for card section titles. */
export const CardTitle = forwardRef<
  HTMLHeadingElement,
  HTMLAttributes<HTMLHeadingElement>
>(({ className, ...props }, ref) => (
  <h3
    ref={ref}
    className={cn("text-sm font-medium text-ink tracking-tight", className)}
    {...props}
  />
));
CardTitle.displayName = "CardTitle";

/** Muted helper text for explaining the card's purpose or data source. */
export const CardDescription = forwardRef<
  HTMLParagraphElement,
  HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => (
  <p
    ref={ref}
    className={cn("text-xs text-ink-muted leading-relaxed", className)}
    {...props}
  />
));
CardDescription.displayName = "CardDescription";

/** Spaced content wrapper for rows, controls, and dense panel bodies. */
export const CardContent = forwardRef<
  HTMLDivElement,
  HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div ref={ref} className={cn("space-y-2", className)} {...props} />
));
CardContent.displayName = "CardContent";
