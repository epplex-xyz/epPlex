<h1 align="center">epPlex | 1st Prize Hyperdrive Hackathon</h1>

epPlex is a protocol for partial program-owned NFTs, epNFTs. I.e. NFTs that has delegated authority to a Solana program.

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

## Integrations
- Burger Game
- Wen-New-Standard



## Deployment
| Name            | Networks       | Responsibility               | Address | Link                                                                                                                                                                                                                  |
|-----------------|----------------|------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| epplex-core     | DEVNET/MAINNET | Main epplex NFT              | `epCoD6BqcNinLvKN3KkY55vk4Kxs3W1JTENs1xqWUTg`     | [DEVNET](https://explorer.solana.com/address/epCoD6BqcNinLvKN3KkY55vk4Kxs3W1JTENs1xqWUTg?cluster=devnet)/[MAINNET](https://explorer.solana.com/address/epCoD6BqcNinLvKN3KkY55vk4Kxs3W1JTENs1xqWUTg?cluster=mainnet) |
| epplex-burger   | DEVNET/MAINNET | For custom NFT logic         | `epBuJysRKuFMMWTWoX6ZKPz5WTZWb98mDqn1emVj84n`     | [DEVNET](https://explorer.solana.com/address/epBuJysRKuFMMWTWoX6ZKPz5WTZWb98mDqn1emVj84n?cluster=devnet)/[MAINNET](https://explorer.solana.com/address/epBuJysRKuFMMWTWoX6ZKPz5WTZWb98mDqn1emVj84n?cluster=mainnet)                                                                                                                                                                                                                   |


## Team Development

### Setup
1. Clone epplex repo
2. Clone sdk repo
3. Ask Bob for local keypairs
    4. Put program keypairs into `target/deploy`
    5. Put local admin keypair into `.local_keys/epplex_PAYER_ADMIN.json`
        6. pubkey address: `LAdmTEtom7qm3ZmchsrqSkZhPdmZaex7oXCamuMHs9F`
5. Run: `sh start.sh` in a terminal
    4. This starts a local validator
6. Run: `sh start.sh` again in another terminal
    6. This deploys the programs into the local validator
7. Set `.env` in SDK repo to localhost `RPC=http://127.0.0.1:8899`
8. Now your local environment is all set up


## Disclaimer
The code has not been audited.

