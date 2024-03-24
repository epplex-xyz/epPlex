use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

// Payer keypair has to sign everything
pub const ADMIN_PUBKEY: Pubkey = pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh");
// pub const ADMIN_PUBKEY: Pubkey = pubkey!("LAdmTEtom7qm3ZmchsrqSkZhPdmZaex7oXCamuMHs9F");

// #[cfg(feature = "mainnet")]
// pub const ADMINS: [Pubkey; 1] = [pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh")];

// #[cfg(not(feature = "mainnet"))]
pub const ADMINS: [Pubkey; 3] = [
    pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh"),
    pubkey!("LAdmTEtom7qm3ZmchsrqSkZhPdmZaex7oXCamuMHs9F"),
    pubkey!("G4QhBg3fF2U7RSwC734ViwL3DeZVrR2TyHMNWHSLwMj"),
];
