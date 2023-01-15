/** @type {import('tailwindcss').Config} */

// tailwind.config.js
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./index.html", "./src/**/*.{html,js,rs}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: colors.amber,
        secondary: colors.teal,
        accent: colors.pink,
      },
      fontFamily: {
        sans: ['Graphik', 'sans-serif'],
        serif: ['Merriweather', 'serif'],
      },
      borderWidth: {
        '1': '1px'
      }
    },
    
  },
  safelist: ["*"],
  plugins: [],
};
