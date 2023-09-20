import localFont from "next/font/local";


export const AlegreyaBold = localFont({
    preload: true,
    display: "swap",
    src: [
        {
            path: '../../public/fonts/alegreya/AlegreyaSansSC-BoldItalic.ttf',
        },
    ],
    fallback: ['Helvetica Neue', 'sans-serif'],
});


export const SatoshiMedium = localFont({
    preload: true,
    display: "swap",
    src: [
        {
            path: '../../public/fonts/satoshi/Satoshi-Medium.woff2',
        },
    ],
    fallback: ['Helvetica Neue', 'sans-serif'],
});

export const SatoshiBold = localFont({
    preload: true,
    display: "swap",
    src: [
        {
            path: '../../public/fonts/satoshi/Satoshi-Bold.woff2',
        },
    ],
    fallback: ['Helvetica Neue', 'sans-serif'],
});