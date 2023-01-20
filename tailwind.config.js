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
          light: "var(--md-color-primary-light)",
          dark: "var(--md-color-primary-dark)",
          on: {
            light: "var(--md-color-on-primary-light) ",
            dark: "var(--md-color-on-primary-dark) ",
          },
          container: {
            light:
              "var(--md-color-primary-container-light) ",
            dark: "var(--md-color-primary-container-dark) ",
            on: {
              light:
                "var(--md-color-on-primary-container-light) ",
              dark: "var(--md-color-on-primary-container-dark) ",
            },
          },
          inverse:
          {
            light:
              "var(--md-color-inverse-primary-light) ",
            dark: "var(--md-color-on-primary-container-dark) ",
          }
        },

        // Secondary tones
        secondary: {
          light: "var(--md-color-secondary-light) ",
          dark: "var(--md-color-secondary-dark) ",
          on: {
            light: "var(--md-color-on-secondary-light) ",
            dark: "var(--md-color-on-secondary-dark) ",
          },
          container: {
            light:
              "var(--md-color-secondary-container-light) ",
            dark: "var(--md-color-secondary-container-dark) ",
            on: {
              light:
                "var(--md-color-on-secondary-container-light) ",
              dark: "var(--md-color-on-secondary-container-dark) ",
            },
          },
        },

        // Tertiary tones
        tertiary: {
          light: "var(--md-color-tertiary-light) ",
          dark: "var(--md-color-tertiary-dark) ",
          on: {
            light: "var(--md-color-on-tertiary-light) ",
            dark: "var(--md-color-on-tertiary-dark) ",
          },
          container: {
            light:
              "var(--md-color-tertiary-container-light) ",
            dark: "var(--md-color-tertiary-container-dark) ",
            on: {
              light:
                "var(--md-color-on-tertiary-container-light) ",
              dark: "var(--md-color-on-tertiary-container-dark) ",
            },
          },
        },

        // Neutral tones (md3 names them as 'surface')
        neutral: {
          light: "var(--md-color-surface-light) ",
          dark: "var(--md-color-surface-dark) ",
          on: {
            inverse: {
              light: "var(--md-color-inverse-on-surface-light) ",
              dark: "var(--md-color-inverse-on-surface-dark) ",
            },
            light: "var(--md-color-on-surface-light) ",
            dark: "var(--md-color-on-surface-dark) ",
          },
          container: {
            light:
              "var(--md-color-neutral-container-light) ",
            dark: "var(--md-color-neutral-container-dark) ",
            on: {
              light:
                "var(--md-color-on-neutral-container-light) ",
              dark: "var(--md-color-on-neutral-container-dark) ",
            },
          },
          // Neutral variant tones
          variant:
          {
            light: "var(--md-color-neutral-light) ",
            dark: "var(--md-color-neutral-dark) ",
            on: {
              light: "var(--md-color-on-neutral-light) ",
              dark: "var(--md-color-on-neutral-dark) ",
            },
            container: {
              light:
                "var(--md-color-surface-variant-light) ",
              dark: "var(--md-color-surface-variant-dark) ",
              on: {
                light:
                  "var(--md-color-on-surface-variant-light) ",
                dark: "var(--md-color-on-surface-variant-dark) ",
              },
            },
          }
        },

        // On Error tones
        error:{
          light:
          "var(--md-color-error-light) ",
        dark: "var(--md-color-error-dark) ",
        on: {
          light:
            "var(--md-color-on-error-light) ",
          dark: "var(--md-color-on-error-dark) ",
        },
        container: {
          light:
            "var(--md-color-error-container-light) ",
          dark: "var(--md-color-error-container-light) ",
          on: {
            light:
              "var(--md-color-on-error-container-light) ",
            dark: "var(--md-color-on-error-container-dark) ",
          },
        },
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
