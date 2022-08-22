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
        header: 50,
        "work-navigation-overlay": 30,
      },
      minWidth: {
        "3/5": "60%",
      },
      gridTemplateColumns: {
        "masonry-lg": "repeat(auto-fill, minmax(16rem, 1fr))",
        "masonry-md": "repeat(auto-fill, minmax(12rem, 1fr))",
        "masonry-sm": "repeat(auto-fill, minmax(8rem, 1fr))",
      },
      gridAutoRows: {
        0: 1,
      },
      height: {
        header: "3.5rem",
      },
    },
  },
  plugins: [],
};
