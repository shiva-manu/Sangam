/**
 * @fileoverview Primary navigation sidebar for the desktop app.
 *
 * The sidebar persists across routes and gives operators stable access to each
 * major dashboard view.
 */
import { NavLink } from "react-router-dom";
import { motion } from "framer-motion";
import {
  Activity,
  BarChart3,
  Boxes,
  Cpu,
  LayoutDashboard,
  ListChecks,
  Settings,
  Terminal,
} from "lucide-react";
import { cn } from "../../lib/cn";

// Ordered primary navigation model. Dashboard uses `end: true` so `/` is an
// exact match; without it React Router would mark Dashboard active for every
// nested route because all paths start at root.
const NAV = [
  { to: "/", label: "Dashboard", icon: LayoutDashboard, end: true },
  { to: "/cluster", label: "Cluster", icon: Boxes },
  { to: "/nodes", label: "Nodes", icon: Cpu },
  { to: "/tasks", label: "Tasks", icon: ListChecks },
  { to: "/runtime", label: "Runtime", icon: Terminal },
  { to: "/analytics", label: "Analytics", icon: BarChart3 },
  { to: "/settings", label: "Settings", icon: Settings },
];

export function Sidebar() {
  return (
    <aside className="relative z-10 w-[220px] shrink-0 border-r border-white/[0.04] bg-bg-surface/40 backdrop-blur-xl flex flex-col">
      {/* Brand logo: glowing activity glyph + product lockup anchors the app
          and reinforces that this is the local Sangam compute mesh. */}
      <div className="px-5 pt-5 pb-6">
        <div className="flex items-center gap-2.5">
          <div className="relative">
            <div className="absolute inset-0 rounded-lg bg-accent-cyan/20 blur-lg" />
            <div className="relative w-8 h-8 rounded-lg bg-gradient-to-br from-accent-cyan/30 to-accent-violet/20 border border-white/10 flex items-center justify-center">
              <Activity
                className="w-4 h-4 text-accent-cyan"
                strokeWidth={2.5}
              />
            </div>
          </div>
          <div>
            <div className="text-sm font-semibold tracking-tight text-ink leading-none">
              Sangam
            </div>
            <div className="text-[10px] uppercase tracking-[0.18em] text-ink-dim mt-1">
              Compute Mesh
            </div>
          </div>
        </div>
      </div>

      {/* Nav */}
      <nav className="flex-1 px-3 space-y-0.5">
        {NAV.map(({ to, label, icon: Icon, end }) => (
          <NavLink key={to} to={to} end={end}>
            {({ isActive }) => (
              <motion.div
                whileHover={{ x: 2 }}
                transition={{ type: "spring", stiffness: 500, damping: 30 }}
                className={cn(
                  "relative flex items-center gap-3 px-3 py-2 rounded-lg",
                  "text-sm font-medium transition-colors",
                  isActive
                    ? "text-ink bg-white/[0.04]"
                    : "text-ink-muted hover:text-ink hover:bg-white/[0.02]",
                )}
              >
                {/* Active indicator: cyan glow line on the left.
                    `layoutId` enables Framer Motion's shared layout animation,
                    so the same indicator appears to glide between links. */}
                {isActive && (
                  <motion.div
                    layoutId="nav-indicator"
                    className="absolute left-0 top-1.5 bottom-1.5 w-[2px] rounded-full bg-accent-cyan shadow-[0_0_8px_rgb(56_189_248_/_0.6)]"
                    transition={{ type: "spring", stiffness: 400, damping: 30 }}
                  />
                )}
                <Icon className="w-4 h-4 shrink-0" />
                <span>{label}</span>
              </motion.div>
            )}
          </NavLink>
        ))}
      </nav>

      {/* Footer: build/version */}
      <div className="px-5 py-4 border-t border-white/[0.04]">
        <div className="text-[10px] uppercase tracking-wider text-ink-dim">
          v0.1.0 · local
        </div>
      </div>
    </aside>
  );
}
