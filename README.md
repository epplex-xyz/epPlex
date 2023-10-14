<h1 align="center">EPPLEX | Hyperdrive Hackathon</h1>

epPlex is a new NFT protocol for ephemeral epNFTs.
I.e. NFTs that can self-destruct permissionlessly.

It is an open-source Infrastructure Primitive & Public Good built on top of Token2022 & Metaplex

## Website
[https://epplex.xyz](https://epplex.xyz/)

## Pitch Deck
[Hyperdrive Pitch](https://epplex.xyz/HyperdrivePitch.pdf)


## Use-cases
To illustrate viable businesses I attempted to build AdDrop ([link to repo](https://github.com/epPlex/AdDrop)):
A mobile app for creators to airdrop epNFTs as ads, and for consumers to view, earn & use ads.
Although the idea probably needs to go through a few more idea iterations.

- Services (built-in time restrictions)
  - NFT as subscriptions
  - Access to services
  - Ticketing
  - Coupons
- Social media apps
  - Snapchat-like apps
- Performance art
  - High-brow artwork for ephemeral enjoyment
- Games where
  - Tamagotchi game where NFT self-destructs if not attended to properly
- NFT collections
  - Utilisng hot-potato mechanisms
  - New holder engagement mechanisms
  - DAO-gated access

## How it works

- Program delegate

### Creation
Token2022
- Permanent Delegate
- CloseAuthority
- TokenMetadata
  - key-value pair: {DestroyTimestamp: unixTimestamp}

Bot infrastructure

NFT rent collector can
Account closing fees

## Deployment

| Description | Address                                       | Link                                                                                                           |
|-------------|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| Devnet      | `BcKkiAcNredLZdQySoHt7okfhDNA32r9mJayjy8cMDdY` | [solexplorer](https://explorer.solana.com/address/BcKkiAcNredLZdQySoHt7okfhDNA32r9mJayjy8cMDdY?cluster=devnet) |
| Mainnet     | TBA                                           | TBA                                                                                                            |


## Future work
- √ epPlex MVP
- Build SDK for developers
- Bot infrastructure for destroying epNFTs
  - Using Clockwork
- METAPLEX integration
  - Token Metadata Program with Token2022 support is an assumed prerequisite for proper epNFT adoption
    - Token2022 audit should be finished by Q4 2023.
  - NOTE: Currently, Token2022 metadata is used, but it probably takes more time to be adopted by wallets
    - I need to investigate how wallets index NFTs, whether it is purely based on MPLX Token Metadata Program.
- State-compression for cheaper minting fees
- Event-based ephemerality e.g. self-destruct
  - when BTC reaches $100k
  - after X amount of transfers
  - other programmable logic
- Immutability


## Tech stack
- NextJS Frontend/Backend (hosted on vercel) for demoing purposes
  - Uses Genesysgo SHDW for NFT metadata
- Anchor Framework for epPlex contract


### Setup

1. Fork and clone the repository.

2. Go to `app` folder and Install dependencies:
```bash
    yarn install
```
3. Start developing
```bash
    yarn dev
```


## Code quality
Please do not expect high code-quality since majority of the code is hacked together.

## Disclaimer
The code has not been audited. Furthermore, as of October 6th 2023, Token2022 is not officially production-ready.

Use at your own risk.

