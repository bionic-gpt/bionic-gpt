/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../web-pages/**/*.rs", "../daisy-rsx/**/*.rs", "typescript/**/*.ts"],
  /** We need this because we use daisy-rsx library */
  safelist: [
    "text-center",
    "z-[1]",
    "w-52",
    "rounded-box",
    "bg-base-100",
    "shadow",
    {
      pattern: /alert*|btn*|menu*|dropdown*|badge*|card*|input*|select*|textarea*|label*|tab*|tooltip*|flex*|text*|overflow*/
    }
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
    require('@tailwindcss/typography')
  ],
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
          "primary": "#3559c7",
          "secondary": "#f6d860",
          "accent": "#37cdbe",
          "neutral": "#3d4451",
          "base-100": "#ffffff",

          "--rounded-box": "5px", // border radius rounded-box utility class, used in card and other large boxes
          "--rounded-btn": "2px", // border radius rounded-btn utility class, used in buttons and similar element
          "--rounded-badge": "1rem", // border radius rounded-badge utility class, used in badges and similar
          "--animation-btn": "none", // duration of animation when you click on button
          "--animation-input": "none", // duration of animation for inputs like checkbox, toggle, radio, etc
          "--btn-focus-scale": "0.95", // scale transform of button when you focus on it
          "--border-btn": "1px", // border width of buttons
          "--tab-border": "1px", // border width of tabs
          "--tab-radius": "0.5rem", // border radius of tabs
        },
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],

          "--rounded-box": "5px", // border radius rounded-box utility class, used in card and other large boxes
          "--rounded-btn": "2px", // border radius rounded-btn utility class, used in buttons and similar element
          "--rounded-badge": "1rem", // border radius rounded-badge utility class, used in badges and similar
          "--animation-btn": "none", // duration of animation when you click on button
          "--animation-input": "none", // duration of animation for inputs like checkbox, toggle, radio, etc
          "--btn-focus-scale": "0.95", // scale transform of button when you focus on it
          "--border-btn": "1px", // border width of buttons
          "--tab-border": "1px", // border width of tabs
          "--tab-radius": "0.5rem", // border radius of tabs
          "--default-border-color": "#e5e7eb", 
        }
      },
    ],
  },
}

