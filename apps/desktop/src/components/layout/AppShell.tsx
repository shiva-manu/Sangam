/**
 * @fileoverview Persistent application shell frame.
 *
 * `AppShell` owns the sidebar, top bar, and routed page outlet.  It stays
 * mounted across route changes so navigation feels continuous and only the
 * page body animates.
 */
import { AnimatePresence, motion } from "framer-motion";
import { Outlet, useLocation } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";

/**
 * Renders the persistent frame every page lives inside.
 *
 * The sidebar and top bar mount once and survive navigation; the outlet below
 * is the only region that changes when React Router swaps pages.
 */
export function AppShell() {
  const location = useLocation();

  return (
    <div className="relative flex h-screen w-screen overflow-hidden">
      <Sidebar />
      <div className="relative z-10 flex-1 flex flex-col min-w-0">
        <TopBar />
        <main className="relative flex-1 overflow-y-auto">
          {/* Gradient overlay: a soft top fade makes content scrolling under
              the fixed top bar feel tucked rather than abruptly clipped. */}
          <div className="pointer-events-none absolute top-0 left-0 right-0 h-6 bg-gradient-to-b from-bg-base/80 to-transparent z-10" />
          {/* mode="wait" guarantees the exiting page finishes its animation
              before the entering page mounts, preventing cross-faded layout
              collisions between dense dashboard panels. */}
          <AnimatePresence mode="wait">
            <motion.div
              key={location.pathname}
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -4 }}
              transition={{ duration: 0.2, ease: "easeOut" }}
              className="px-6 py-6"
            >
              <Outlet />
            </motion.div>
          </AnimatePresence>
        </main>
      </div>
    </div>
  );
}
