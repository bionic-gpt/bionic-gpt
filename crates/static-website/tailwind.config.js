/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      typography: (theme) => ({
        DEFAULT: {
          css: {
            "pre.hljs-copy-wrapper": {
              "padding-top": "0",
              //"padding-bottom": "0",
              "padding-inline-end": "0",
              "padding-inline-start": "0"
            },
          },
        },
      }),
    },
  },
  plugins: [
    require('daisyui'),
    require('@tailwindcss/typography')
  ],
};
