/** @type {import('tailwindcss').Config} */

// tailwind.config.js
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./index.html", "./src/**/*.{html,js,rs}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: colors.purple,
        secondary: colors.pink,
        accent: colors.emerald,
      },
    },
  },
  safelist: ["*"],
  plugins: [],
};
