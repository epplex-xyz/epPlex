use std::ops::Add;
use anchor_lang::prelude::borsh::BorshDeserialize;
use spl_token_2022::extension::BaseStateWithExtensions;
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenRenewParams)]
pub struct TokenRenew<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
    )]
    /// CHECK
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        token::mint = mint,
        token::authority = authority,
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    // TODO: test not authority
    // Signer cus need to pay
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenRenewParams {
    renew_terms: u16,
}

impl TokenRenew<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenRenewParams,
    ) -> Result<()> {


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

    pub fn actuate(ctx: Context<Self>, _params: &TokenRenewParams) -> Result<()> {
        // TODO
        // How to enforce that you can only renew once?
        // you would need to store the old destroy_timestamp somewhere to maintain a range.

        // TODO need to accept payments
        // param should simply
        // should compute the costs


        // TODO need to turn this into a helper function
        let buffer = ctx.accounts.mint.to_account_info();
        let expiry_date;
        {

            let mint_data = buffer.try_borrow_data()?;
            let state = spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
            let metadata_bytes = state.get_extension_bytes::<TokenMetadata>().unwrap();
            let fetched_metadata = TokenMetadata::try_from_slice(metadata_bytes).unwrap();

            let temp = fetched_metadata.additional_metadata[0].1.clone();
            expiry_date = temp.parse::<i64>().unwrap();

            msg!("Destroy timestamp: {}", temp);
            let now = Clock::get().unwrap().unix_timestamp;
            if now > expiry_date {
                msg!("ExpiryDate already exceeded");
            }

            msg!("new timestamp: {}", expiry_date.add(ONE_WEEK));
        }

        update_token_metadata(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO rethink this
            &ctx.accounts.authority.to_account_info(), // who is allowed to make changes here? Changes have to go through program?
            spl_token_metadata_interface::state::Field::Key(EXPIRY_FIELD.to_string()),
            expiry_date.add(ONE_WEEK).to_string()
        )?;

        Ok(())
    }
}
