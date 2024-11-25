/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",        // Archivo HTML principal
    "./src/**/*.{rs,html}" // Archivos Rust y HTML en src/
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
