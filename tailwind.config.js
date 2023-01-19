/** @type {import('tailwindcss').Config} */

// tailwind.config.js
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./index.html", "./src/**/*.{html,js,rs}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {

        // Primary tones
        primary: {
          light: "rgba(var(--md-sys-color-primary-light) / <alpha-value>)",
          dark: "rgba(var(--md-sys-color-primary-dark) / <alpha-value>)",
          on: {
            light: "rgba(var(--md-sys-color-on-primary-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-on-primary-dark) / <alpha-value>)",
          },
          container: {
            light:
              "rgba(var(--md-sys-color-primary-container-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-primary-container-dark) / <alpha-value>)",
            on: {
              light:
                "rgba(var(--md-sys-color-on-primary-container-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-on-primary-container-dark) / <alpha-value>)",
            },
          },
          inverse:
          {
            light:
              "rgba(var(--md-sys-color-inverse-primary-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-on-primary-container-dark) / <alpha-value>)",
          }
        },

        // Secondary tones
        secondary: {
          light: "rgba(var(--md-sys-color-secondary-light) / <alpha-value>)",
          dark: "rgba(var(--md-sys-color-secondary-dark) / <alpha-value>)",
          on: {
            light: "rgba(var(--md-sys-color-on-secondary-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-on-secondary-dark) / <alpha-value>)",
          },
          container: {
            light:
              "rgba(var(--md-sys-color-secondary-container-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-secondary-container-dark) / <alpha-value>)",
            on: {
              light:
                "rgba(var(--md-sys-color-on-secondary-container-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-on-secondary-container-dark) / <alpha-value>)",
            },
          },
        },

        // Tertiary tones
        tertiary: {
          light: "rgba(var(--md-sys-color-tertiary-light) / <alpha-value>)",
          dark: "rgba(var(--md-sys-color-tertiary-dark) / <alpha-value>)",
          on: {
            light: "rgba(var(--md-sys-color-on-tertiary-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-on-tertiary-dark) / <alpha-value>)",
          },
          container: {
            light:
              "rgba(var(--md-sys-color-tertiary-container-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-tertiary-container-dark) / <alpha-value>)",
            on: {
              light:
                "rgba(var(--md-sys-color-on-tertiary-container-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-on-tertiary-container-dark) / <alpha-value>)",
            },
          },
        },

        // Neutral tones (md3 names them as 'surface')
        neutral: {
          light: "rgba(var(--md-sys-color-surface-light) / <alpha-value>)",
          dark: "rgba(var(--md-sys-color-surface-dark) / <alpha-value>)",
          on: {
            inverse: {
              light: "rgba(var(--md-sys-color-inverse-on-surface-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-inverse-on-surface-dark) / <alpha-value>)",
            },
            light: "rgba(var(--md-sys-color-on-surface-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-on-surface-dark) / <alpha-value>)",
          },
          container: {
            light:
              "rgba(var(--md-sys-color-neutral-container-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-neutral-container-dark) / <alpha-value>)",
            on: {
              light:
                "rgba(var(--md-sys-color-on-neutral-container-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-on-neutral-container-dark) / <alpha-value>)",
            },
          },
          // Neutral variant tones
          variant:
          {
            light: "rgba(var(--md-sys-color-neutral-light) / <alpha-value>)",
            dark: "rgba(var(--md-sys-color-neutral-dark) / <alpha-value>)",
            on: {
              light: "rgba(var(--md-sys-color-on-neutral-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-on-neutral-dark) / <alpha-value>)",
            },
            container: {
              light:
                "rgba(var(--md-sys-color-surface-variant-light) / <alpha-value>)",
              dark: "rgba(var(--md-sys-color-surface-variant-dark) / <alpha-value>)",
              on: {
                light:
                  "rgba(var(--md-sys-color-on-surface-variant-light) / <alpha-value>)",
                dark: "rgba(var(--md-sys-color-on-surface-variant-dark) / <alpha-value>)",
              },
            },
          }
        },

        // Back
      },
      fontFamily: {
        sans: ["Graphik", "sans-serif"],
        serif: ["Merriweather", "serif"],
      },
      borderWidth: {
        1: "1px",
      },
    },
  },
  safelist: ["*"],
  plugins: [],
};
