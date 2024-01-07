class TokenMetadata {
    update_authority: string; // Assuming OptionalNonZeroPubkey is a string
    mint: string;
    name: string;
    symbol: string;
    uri: string;
    additional_metadata: [string, string][];

    static schema = new Map([
        [
            TokenMetadata,
            {
                kind: 'struct',
                fields: [
                    ['update_authority', 'string'],
                    ['mint', 'string'],
                    ['name', 'string'],
                    ['symbol', 'string'],
                    ['uri', 'string'],
                    ['additional_metadata', ['array', ['tuple', ['string', 'string']]]],
                ],
            },
        ],
    ]);

    static decode(buffer: Buffer): TokenMetadata {
        const deserialized = Borsh.deserializeUnchecked(
            this.schema,
            TokenMetadata,
            buffer
        );
        return deserialized as TokenMetadata;
    }
}