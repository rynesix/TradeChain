/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,jsx,ts,tsx}"
    ],
    theme: {
        extend: {
            fontFamily: {
                inter: ['Inter', 'sans-serif'],
            },
            colors: {
                background: '#111113',
                card: '#1b1b1d',
                primary: '#00FF9F',
                border: '#2c2c2e'
            }
        }
    },
    plugins: [],
};
