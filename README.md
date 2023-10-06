<h1 align="center">EPPLEX | Hyperdrive Hackathon</h1>

epPlex is a new NFT protocol for ephemeral NFTs. I.e. NFTs that can self-destruct permissionlessly.

It is an open-source Infrastructure Primitive & Public Good built on top of Token2022.

## Website
[https://silk-street-frontend.vercel.app/](https://silk-street-frontend.vercel.app/) (responsiveness is bad)


## Use-cases
- AdDrop is using epNFTs
- Snapchat-like applications
- Performance art
- Games where self-destruction is relevant

## How it works
Creation goes through the epplex contract

Token2022
- Permanent Delegate
- CloseAuthority
- TokenMetadata
  - key-value pair: {DestroyTimestamp: unixTimestamp}

Bot infrastructure
Account closing fees

## Deployment

| Description | Address                                       | Link                                                                                                           |
|-------------|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| DEVNET      | `BcKkiAcNredLZdQySoHt7okfhDNA32r9mJayjy8cMDdY` | [solexplorer](https://explorer.solana.com/address/BcKkiAcNredLZdQySoHt7okfhDNA32r9mJayjy8cMDdY?cluster=devnet) |
| MAINNET     | TBA                                           | TBA                                                                                                            |


## Future work
- Bot infra

- Metaplex integration
  - Support for Token2022 is a prerequisite for wider epNFT adoption

- Easy-to-use SDK

- State-compression
    - For cheaper minting fees

- Event-based ephemerality
  - e.g. self-destruct when BTC reaches $100k



## Tech stack
- NextJS Frontend/Backend (hosted on vercel) for demoing purposes
- Anchor Framework for epPlex contract

## Disclaimer
The code has not been audited. Furthermore, as of 06/10/23 Token2022 is not officially production-ready.

Use at your own risk.


## Credits
- [dReader](https://github.com/d-reader-organization) for providing NestJs inspiration
- [Cubik](https://github.com/cubik-so) for providing monorepo inspiration