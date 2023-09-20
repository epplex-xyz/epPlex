import '../src/styles/globals.scss';
import React from "react";
import type { AppProps } from 'next/app';
import {CustomThemeProvider} from "../src/providers/CustomThemeProvider";
import createEmotionCache from "../src/assets/createEmotionCache";
import { EmotionCache} from '@emotion/react';
import { Analytics } from '@vercel/analytics/react';
import Head from "next/head";
import {DefaultSeo } from "next-seo";
import SEO from "../next-seo-config";
import * as font from "../src/assets/fonts";
import { AlegreyaBold } from "../src/assets/fonts";
import MyWalletProvider from "../src/providers/MyWalletProvider";

// Client-side cache, shared for the whole session of the user in the browser.
const clientSideEmotionCache = createEmotionCache();

interface MyAppProps extends AppProps {
    emotionCache?: EmotionCache;
}

// _app runs on both client and server
function App(props: MyAppProps) {
    const { Component, emotionCache = clientSideEmotionCache, pageProps } = props;

    return (
        <CustomThemeProvider cache={emotionCache}>
            <MyWalletProvider>
                {/* SEO */}
                <Head>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                </Head>
                <DefaultSeo {...SEO} />

                {/* Vercel */}
                <Analytics />

                {/* Ensures proper initial font loading */}
                <div className={`
                    ${font.AlegreyaBold.className}
                    ${font.SatoshiBold.className}
                    ${font.SatoshiMedium.className}
                `}>
                    {process.env.NODE_ENV === "production" ?
                        <Component {...pageProps}/>
                        :
                        <React.StrictMode>
                            <Component {...pageProps}/>
                        </React.StrictMode>
                    }
                </div>
            </MyWalletProvider>
        </CustomThemeProvider>
    );
}

export default App;
