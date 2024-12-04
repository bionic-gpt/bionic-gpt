/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}"],
  theme: {
    extend: {
      
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('daisyui')
  ],
}