import localFont from "next/font/local";

export const BasteleurBold = localFont({
    preload: true,
    display: "swap",
    src: [
        {
            path: '../../public/fonts/basteleur/Basteleur-Bold.woff2',
        },
    ],
    fallback: ['BasteleurBold', 'Helvetica Neue'],
});


export const SatoshiMedium = localFont({
    preload: true,
    display: "swap",
    src: [
        {
            path: '../../public/fonts/satoshi/Satoshi-Medium.ttf',
        },
    ],
    fallback: ['SatoshiMedium', 'Helvetica Neue', 'sans-serif'],
});

export const SatoshiBold = localFont({
    preload: true,
    display: "swap",
    src: [
        {
            path: '../../public/fonts/satoshi/Satoshi-Bold.woff',
        },
    ],
    fallback: ['SatoshiBold', 'Helvetica Neue', 'sans-serif'],
});