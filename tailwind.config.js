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
        // white opacity(for use bg)
        "white-opacity-10": "var(--color-white-opacity-10)",
        "white-opacity-20": "var(--color-white-opacity-20)",
        "white-opacity-30": "var(--color-white-opacity-30)",
        "white-opacity-50": "var(--color-white-opacity-50)",
        "white-opacity-70": "var(--color-white-opacity-70)",
        "white-opacity-80": "var(--color-white-opacity-80)",
        "white-opacity-90": "var(--color-white-opacity-90)",
      },
      zIndex: {
        "dialog-overlay": 104,
        dialog: 105,
        "list-box-overlay": 109,
        "list-box": 110,
        header: 50,
        "work-navigation-overlay": 30,
        "artist-navigation-overlay": 30,
      },
      minWidth: {
        "3/5": "60%",
      },
      maxWidth: {
        "search-tags": "calc(100% - 12rem)",
      },
      gridTemplateColumns: {
        "masonry-lg": "repeat(auto-fill, minmax(16rem, 1fr))",
        "masonry-md": "repeat(auto-fill, minmax(12rem, 1fr))",
        "masonry-sm": "repeat(auto-fill, minmax(8rem, 1fr))",
        "image-list": "repeat(auto-fill, minmax(5rem, 1fr))",
        "top-header": "min-content minmax(0, 1fr) repeat(3, min-content)",
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
