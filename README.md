<h1 align="center">epPlex | Hyperdrive Hackathon</h1>

epPlex is a protocol for ephemeral epNFTs.
I.e. NFTs that can self-destruct permissionlessly.

It is an open-source Infrastructure Primitive & Public Good built on top of Token2022 & Metaplex.

Submitted tracks:
- Public Goods (was not a choice in the submission form)
- Crypto Infrastructure
- Gaming & Entertainment


## Links
[https://epplex.xyz](https://epplex.xyz/)
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

### epNFT representation
An epNFT is currently represented as a Token2022 token with the following extensions
- `Permanent Delegate`
- `CloseAuthority`
- `TokenMetadata` key-value pairs:
    - `DestroyTimestamp`: unixTimestamp
    - `name`: string
    - `symbol`: string
    - `uri`: string

The `uri` field points to the off-chain JSON object with image URL and metadata.

Ideally it would use the MPLX Token Metadata Program for wider ecosystem compatibility.
Rumour has it that Token2022 support for MPLX Token Metadata Program is coming end of Oct 2023.

OBS: I need to double check whether or not the supply is actually fixed to 1 and no Mint Authority exists.

### Global Program Delegate
The Program Delegate PDA is assigned elevated privileges through the Token2022 extensions.
This enables the program to destroy epNFTs on behalf of the owner.

### epNFT lifecycle
1. User submits create instruction with TokenMetadata
   2. epNFT is created through CPI into Token2022 program
   3. Program Delegate is assigned `Permanent Delegate` and `CloseAuthority` privileges
2. Anyone can submit destroy instruction on the epNFT
   3. If `current_timestamp <= destroy_timestamp` then fail the transaction
   4. Otherwise tx succeeds, where the Program Delegate acts as the authority to burn & close the epNFT.

### Rent collection
Currently, epPlex collects all the epNFT rent-exemption through the Program Delegate.
Although, it can easily be modified to be: instruction invoker collects the rent.

### Bot infrastructure
I currently have not yet implemented any bot infrastructure for destroying epNFTs.
One of the reasons is that this could be offloaded to arbitrageurs (rent farmers). The other reason is lack of time.
It would be cool to have Open-Clockwork power this on-chain.

## Future work
- √ epPlex MVP
- Build SDK for developers
- Bot infrastructure for destroying epNFTs
    - Using Open-Clockwork
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


## Deployment

| Description | Address                                       | Link                                                                                                           |
|-------------|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| Devnet      | `BcKkiAcNredLZdQySoHt7okfhDNA32r9mJayjy8cMDdY` | [solexplorer](https://explorer.solana.com/address/BcKkiAcNredLZdQySoHt7okfhDNA32r9mJayjy8cMDdY?cluster=devnet) |
| Mainnet     | TBA                                           | TBA                                                                                                            |




## Development

### Tech stack
- NextJS Frontend/Backend (hosted on vercel) for demoing purposes
  - Uses Genesysgo SHDW for NFT metadata
- Anchor Framework for epPlex contract

### Folder structure
    ├── app                                 # NextJS Frontend
    ├── programs                            # epPlex Solana program
    └── scripts                             # Scripts for testing purposes, althoguh I probably should have kept a test-suite

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
Please do not expect high code-quality since majority of the code is hacked together. For the sake of achieving a working MVP, code quality has been neglected.

## Disclaimer
The code has not been audited. Furthermore, as of October 6th 2023, Token2022 is not officially production-ready.

Use at your own risk.

