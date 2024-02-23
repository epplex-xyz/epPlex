use crate::*;
// use anchor_spl::token_interface::{Mint as MintInterface};
use anchor_lang::solana_program::pubkey;


#[derive(Accounts)]
#[instruction(params: CollectionCloseParams)]
pub struct CollectionClose<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [
            SEED_COLLECTION_CONFIG,
            params.collection_id.to_le_bytes().as_ref()
        ],
        bump = collection_config.bump,
    )]
    pub collection_config: Account<'info, CollectionConfig>,

    // #[account(
    //     mut,
    //     seeds = [
    //         SEED_COLLECTION_MINT,
    //         params.collection_id.to_le_bytes().as_ref()
    //     ],
    //     bump,
    // )]
    // pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        constraint = [pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh")].contains(
            &payer.key()
        )
    )]
    pub payer: Signer<'info>,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCloseParams {
    collection_id: u64
}

impl CollectionClose<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &CollectionCloseParams
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: CollectionCloseParams) -> Result<()> {
        Ok(())
    }
}


