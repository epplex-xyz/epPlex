const defaultSEOConfig = {
    title: "epPlex",
    description: "epPlex | A novel Solana NFT protocol",
    cannonical: "https://www.epplex.xyz/",
    openGraph: {
        type: "website",
        url: "https://www.epplex.xyz/",
        // This is the first line of discord link
        site_name: "epPlex",
        // Second line, blue text
        title: "epPlex | A novel Solana NFT protocol",
        description: "Home for ephemeral NFTs (epNFTs)",
        images: [
            {
                url: "https://epplex.xyz/logos/logo.png",
                width: 630,
                height: 630,
                alt: "Website logo",
            },
        ],
        profile: {
            firstName: "epPlex",
            lastName: "xyz",
            username: "epplex_xyz",
            gender: "unisex",
        },
    },
    twitter: {
        handle: "@epplex_xyz",
        site: "@epplex_xyz",
        cardType: "summary_large_image",
    },
};

export default defaultSEOConfig;
