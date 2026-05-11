/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Sangam palette: deep space backgrounds with cyan/violet accents.
        // CSS variables (defined in index.css) so we could theme later.
        bg: {
          base: "rgb(var(--bg-base) / <alpha-value>)",
          surface: "rgb(var(--bg-surface) / <alpha-value>)",
          elevated: "rgb(var(--bg-elevated) / <alpha-value>)",
        },
        line: "rgb(var(--line) / <alpha-value>)",
        ink: {
          DEFAULT: "rgb(var(--ink) / <alpha-value>)",
          muted: "rgb(var(--ink-muted) / <alpha-value>)",
          dim: "rgb(var(--ink-dim) / <alpha-value>)",
        },
        accent: {
          cyan: "rgb(var(--accent-cyan) / <alpha-value>)",
          violet: "rgb(var(--accent-violet) / <alpha-value>)",
          green: "rgb(var(--accent-green) / <alpha-value>)",
          amber: "rgb(var(--accent-amber) / <alpha-value>)",
          red: "rgb(var(--accent-red) / <alpha-value>)",
        },
      },
      fontFamily: {
        sans: [
          "Inter",
          "ui-sans-serif",
          "system-ui",
          "-apple-system",
          "Segoe UI",
          "sans-serif",
        ],
        mono: [
          "JetBrains Mono",
          "Geist Mono",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "monospace",
        ],
      },
      boxShadow: {
        // Soft inner glow used on glass panels.
        glow: "inset 0 1px 0 0 rgb(255 255 255 / 0.06), 0 1px 0 0 rgb(255 255 255 / 0.02)",
        "glow-lg":
          "inset 0 1px 0 0 rgb(255 255 255 / 0.08), 0 8px 32px -8px rgb(0 0 0 / 0.4)",
      },
      animation: {
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "fade-in": "fadeIn 0.4s ease-out forwards",
        shimmer: "shimmer 2.4s linear infinite",
        "ping-soft": "pingSoft 2s cubic-bezier(0, 0, 0.2, 1) infinite",
      },
      keyframes: {
        fadeIn: {
          from: { opacity: 0, transform: "translateY(4px)" },
          to: { opacity: 1, transform: "translateY(0)" },
        },
        shimmer: {
          "0%": { transform: "translateX(-100%)" },
          "100%": { transform: "translateX(100%)" },
        },
        pingSoft: {
          "0%": { transform: "scale(1)", opacity: 0.8 },
          "75%, 100%": { transform: "scale(1.8)", opacity: 0 },
        },
      },
    },
  },
  plugins: [],
};
