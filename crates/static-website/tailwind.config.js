/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}"],
  plugins: [
    require('@tailwindcss/typography'),
  ],
}