use std::ops::Add;
use anchor_lang::prelude::borsh::BorshDeserialize;
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenRenewParams)]
pub struct TokenRenew<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
    )]
    /// CHECK
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    // TODO check that this is in fact a token account for the mint
    #[account(
        mut
    )]
    /// CHECK
    pub token_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenRenewParams {
    renew_terms: u16,
}

impl TokenRenew<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        _params: &TokenRenewParams,
    ) -> Result<()> {
        // TODO need to turn this into a helper function
        // let metadata_bytes = ctx.accounts.mint.try_borrow().get_extension_bytes::<TokenMetadata>().unwrap();
        // let fetched_metadata = TokenMetadata::try_from_slice(metadata_bytes).unwrap();
        // let destroy_timestamp = fetched_metadata.additional_metadata[0].1.clone();
        // let data_bytes = ctx.accounts.mint.try_borrow_data()?;
        // let (_, metadata_bytes) = data_bytes.split_at(METADATA_OFFSET);
        // let metadata: Metadata = Metadata::try_from_slice(metadata_bytes)?;
        // let destroy_timestamp = metadata.destroy_timestamp_value.parse::<i64>().unwrap();

        // let now = Clock::get().unwrap().unix_timestamp;
        // if now < destroy_timestamp {
        //     return err!(EphemeralityError::DestroyTimestampNotExceeded);
        // }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: &TokenRenewParams) -> Result<()> {
        // TODO can I change token metadata without any auth?

        // Also need to be able to transfer
        // param should simply
        // should compute the costs


        // let data_bytes = ctx.accounts.mint.try_borrow_data()?;
        // let (_, metadata_bytes) = data_bytes.split_at(METADATA_OFFSET);
        // let mut metadata: Metadata = Metadata::try_from_slice(metadata_bytes)?;
        //
        // let current = metadata.destroy_timestamp_value.parse::<i64>().unwrap();
        // // this needs to be from collection config
        // let new = current.add(1000000).to_string();
        // metadata.destroy_timestamp_value = new;

        Ok(())
    }
}
