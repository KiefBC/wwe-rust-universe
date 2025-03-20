/** @type {import('tailwindcss').Config} */
export default {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
