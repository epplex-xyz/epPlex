import '../src/styles/globals.scss';
import React from "react";
import type { AppProps } from 'next/app';
import {CustomThemeProvider} from "../src/providers/CustomThemeProvider";
import createEmotionCache from "@styles/createEmotionCache";
import { EmotionCache} from '@emotion/react';
import { Analytics } from '@vercel/analytics/react';
import Head from "next/head";
import {DefaultSeo } from "next-seo";
import SEO from "../next-seo-config";
import * as font from "@styles/fonts";
import MyWalletProvider from "../src/providers/MyWalletProvider";
import ProgramApisProvider from "../src/providers/ProgramApisProvider";
import { MyToaster } from "@components/Container/Toaster";

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
                <ProgramApisProvider>
                    {/* SEO */}
                    <Head>
                        <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    </Head>
                    <DefaultSeo {...SEO} />

                    {/* Vercel */}
                    <Analytics />

                    <MyToaster/>

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
                </ProgramApisProvider>
            </MyWalletProvider>
        </CustomThemeProvider>
    );
}

export default App;
