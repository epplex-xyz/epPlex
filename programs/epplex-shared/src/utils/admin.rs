use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

// Payer keypair has to sign everything
pub const ADMIN_PUBKEY: Pubkey = pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh");

// #[cfg(feature = "mainnet")]
pub const ADMINS: [Pubkey; 3] = [
    pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh"),
    pubkey!("MA1NqUiWSgJz4VDXjPFfNoDWqBBRpMDnT4vxEnt9qbv"),
    pubkey!("gameAbxppNoqMi4fHHVbQuj9RoYLksh1kX41EJQHTkd"),
];