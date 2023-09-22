import {PublicKey} from "@solana/web3.js";

// From Metaplex
type JsonMetadata<Uri = string> = {
    name?: string;
    symbol?: string;
    description?: string;
    seller_fee_basis_points?: number;
    image?: Uri;
    animation_url?: Uri;
    external_url?: Uri;
    attributes?: Array<{
        trait_type?: string;
        value?: string;
        [key: string]: unknown;
    }>;
    properties?: {
        creators?: Array<{
            address?: string;
            share?: number;
            [key: string]: unknown;
        }>;
        files?: Array<{
            type?: string;
            uri?: Uri;
            [key: string]: unknown;
        }>;
        [key: string]: unknown;
    };
    collection?: {
        name?: string;
        family?: string;
        [key: string]: unknown;
    };
    [key: string]: unknown;
};

// https://docs.metaplex.com/programs/token-metadata/changelog/v1.0
export function makeJson(
    imageUri: string,
    name: string,
    symbol: string,
    payer: PublicKey,
    attributes: {trait_type: string, value: string}[]
): JsonMetadata {
    const metadata: JsonMetadata = {
        "name": name,
        "symbol": symbol,
        "description": "",
        "seller_fee_basis_points": 500,
        "image": imageUri,
        "attributes": attributes,
        "external_url": "https://www.epplex.xyz",
        "collection": {},
        "properties": {
            "creators": [
                {
                    "address": payer.toString(),
                    "share": 100
                },
            ],
            "category": "image",
            "files": [
                {
                    "uri": imageUri,
                    "type": "image/png"
                }
            ]
        }
    };

    return metadata;
}