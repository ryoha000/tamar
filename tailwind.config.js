module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        background: "var(--color-background)",
        primary: "var(--color-primary)",
        secondary: "var(--color-secondary)",
        accent: "var(--color-accent)",
        error: "var(--color-error)",
        white: "var(--color-white)",
        text: "var(--color-text)",
      },
      zIndex: {
        "dialog-overlay": 104,
        dialog: 105,
        "list-box-overlay": 109,
        "list-box": 110,
      },
    },
  },
  plugins: [],
};
