const Color = require('color')
const alpha = (clr, val) => Color(clr).alpha(val).rgb().string()
const lighten = (clr, val) => Color(clr).lighten(val).rgb().string()
const darken = (clr, val) => Color(clr).darken(val).rgb().string()

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src-ui/**/*.{html,tsx}"],
    theme: {
        colors: {
            "blurple": "#5865F2",
            "blurple-lighter": lighten("#5865F2", 0.1),
            "dark-black": "#2C2F33", // DARK_BUT_NOT_BLACK
            "not-black": "#23272A", // NOT_QUITE_BLACK
            "white": "#FFFFFF",
            "red": "#ED4245",
            "green": "#57F287",
            "black": "#23272A",
        },
    },
    plugins: [],
}
