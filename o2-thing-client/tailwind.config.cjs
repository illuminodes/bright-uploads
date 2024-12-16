/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['**/*.{html,rs}', '../**/*.{html,rs}'],
    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
    ],
    theme: {
        extend: {
            fontFamily: {
                roboto: ['Roboto', 'sans-serif'],
                "roboto-bold": ['Roboto-Bold', 'sans-serif'],
            },
            colors: {
                'pv-dark': '#5D5ABF',
                'pv-blue': '#5752D9',
                'pv-light': '#94F2F2',
                'gray': '#F2F2F2',
                'black': '#0D0D0D',
            },
            animation: {
                fadeIn: 'fadeIn 0.5s ease-out',
                fadeOut: 'fadeOut 0.5s ease-out',
                slideInRight: 'slideInRight 0.4s ease-out',
                slideOutRight: 'slideOutRight 0.4s ease-out',
                slideInLeft: 'slideInLeft 0.4s ease-out',
                slideOutLeft: 'slideOutLeft 0.4s ease-out',
            },
            keyframes: {
                fadeIn: {
                    '0%': { opacity: '0' },
                    '100%': { opacity: '1' },
                },
                fadeOut: {
                    '0%': { opacity: '1' },
                    '100%': { opacity: '0' },
                },
                slideInRight: {
                    '0%': { transform: 'translateX(100%)' },
                    '100%': { transform: 'translateX(0)' },
                },
                slideOutRight: {
                    '0%': { transform: 'translateX(0)' },
                    '100%': { transform: 'translateX(100%)' },
                },
                slideInLeft: {
                    '0%': { transform: 'translateX(-100%)' },
                    '100%': { transform: 'translateX(0)' },
                },
                slideOutLeft: {
                    '0%': { transform: 'translateX(0)' },
                    '100%': { transform: 'translateX(-100%)' },
                },
                spinner: {
                    '50%': { transform: 'rotate(180deg)' },
                    '100%': { transform: 'rotate(360deg)' },
                },
            },
        },
    },
};
