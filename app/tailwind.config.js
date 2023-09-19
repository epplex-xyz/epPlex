module.exports = {
    corePlugins: {
        // Remove Tailwind CSS's preflight style so it can use the MUI's preflight instead (CssBaseline).
        preflight: false,
    },
    mode: 'jit',
    content: [
        "./src/components/**/*.{js,jsx,ts,tsx}",
        "./src/content/**/*.{js,jsx,ts,tsx}",
        "./src/providers/**/*.{js,jsx,ts,tsx}",
        "./pages/**/*.{js,jsx,ts,tsx}",
    ],
    theme: {
        screens: {
            'sm': '600px',
            // => @media (min-width: 600px) { ... }

            'md': '900px',
            // => @media (min-width: 900px) { ... }

            'lg': '1200px',
            // => @media (min-width: 1200px) { ... }

            'xl': '1536px',
            // => @media (min-width: 1536px) { ... }
        },
        extend: {},
    },
    plugins: [],
};