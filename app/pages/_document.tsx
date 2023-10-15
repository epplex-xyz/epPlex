// Inspired by https://github.dev/mui/material-ui/tree/master/examples/nextjs-with-typescript
import Document, { Html, Head, Main, NextScript } from "next/document";
import React from "react";
import createEmotionCache from "@styles/createEmotionCache";
import createEmotionServer from "@emotion/server/create-instance";
import theme from "@styles/theme";

// _document.tsx is only rendered on the server
class MyDocument extends Document {
    render() {
        return (
            <Html lang="en">
                <Head>
                    <meta name="theme-color" content={theme("light").palette.primary.main} />
                    <meta charSet="utf-8" />
                    <link rel="icon" href="/logos/newLogo.png" />
                    <link rel="apple-touch-icon" sizes="180x180" href="/logos/newLogo.png" />
                    <link rel="manifest" href="/manifest.json" />
                    <meta name="emotion-insertion-point" content="" />
                    {(this.props as any).emotionStyleTags}

                </Head>
                <body>
                    <Main />
                    <NextScript />
                </body>
            </Html>
        );
    }
}

MyDocument.getInitialProps = async (ctx) => {
    // Resolution order
    //
    // On the server:
    // 1. app.getInitialProps
    // 2. page.getInitialProps
    // 3. document.getInitialProps
    // 4. app.render
    // 5. page.render
    // 6. document.render
    //
    // On the server with error:
    // 1. document.getInitialProps
    // 2. app.render
    // 3. page.render
    // 4. document.render
    //
    // On the client
    // 1. app.getInitialProps
    // 2. page.getInitialProps
    // 3. app.render
    // 4. page.render

    // Render app and page and get the context of the page with collected side effects.
    // const sheets = new ServerStyleSheets();
    const originalRenderPage = ctx.renderPage;

    const cache = createEmotionCache();
    const { extractCriticalToChunks } = createEmotionServer(cache);

    ctx.renderPage = () =>
        originalRenderPage({
            enhanceApp: (App: any) =>
                function EnhanceApp(props) {
                    return <App emotionCache={cache} {...props}/>;
                },
        });

    const initialProps = await Document.getInitialProps(ctx);

    // This is important. It prevents Emotion to render invalid HTML.
    // See https://github.com/mui/material-ui/issues/26561#issuecomment-855286153
    const emotionStyles = extractCriticalToChunks(initialProps.html);
    const emotionStyleTags = emotionStyles.styles.map((style) => (
        <style
            data-emotion={`${style.key} ${style.ids.join(" ")}`}
            key={style.key}
            // eslint-disable-next-line react/no-danger
            dangerouslySetInnerHTML={{ __html: style.css }}
        />
    ));

    return {
        ...initialProps,
        emotionStyleTags,
    };
};

export default MyDocument;
