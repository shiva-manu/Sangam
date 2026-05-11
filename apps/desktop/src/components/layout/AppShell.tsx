import { AnimatePresence, motion } from "framer-motion";
import { Outlet, useLocation } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";

/// The shell every page lives inside. The sidebar + top bar mount once
/// and persist across navigations; only the routed children animate
/// in/out via AnimatePresence so transitions feel snappy.
export function AppShell() {
  const location = useLocation();

  return (
    <div className="relative flex h-screen w-screen overflow-hidden">
      <Sidebar />
      <div className="relative z-10 flex-1 flex flex-col min-w-0">
        <TopBar />
        <main className="relative flex-1 overflow-y-auto">
          {/* Soft top fade so content scrolling under the top bar feels
              tucked rather than abruptly clipped. */}
          <div className="pointer-events-none absolute top-0 left-0 right-0 h-6 bg-gradient-to-b from-bg-base/80 to-transparent z-10" />
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
