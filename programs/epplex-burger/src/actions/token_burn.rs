use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use epplex_shared::Token2022;

#[derive(Accounts)]
#[instruction(params: TokenBurnParams)]
pub struct TokenBurn<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    // TODO fix
    // #[account(
    //     mut,
    //     seeds = [SEED_PROGRAM_DELEGATE],
    //     bump = program_delegate.bump,
    // )]
    // pub program_delegate: Account<'info, ProgramDelegate>,
    #[account()]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    // TODO check that this is in fact a token account for the mint
    #[account(
        mut
    )]
    /// CHECK
    pub token_account: AccountInfo<'info>,

    // TODO put constraint on payer
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBurnParams {}

impl TokenBurn<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenBurnParams,
    ) -> Result<()> {
        // let data_bytes = ctx.accounts.mint.try_borrow_data()?;
        // let (_, metadata_bytes) = data_bytes.split_at(METADATA_OFFSET);
        // let metadata: Metadata = Metadata::try_from_slice(metadata_bytes)?;
        // let destroy_timestamp = metadata.destroy_timestamp_value.parse::<i64>().unwrap();
        //
        // let now = Clock::get().unwrap().unix_timestamp;
        // if now < destroy_timestamp {
        //     return err!(EphemeralityError::DestroyTimestampNotExceeded);
        // }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &TokenBurnParams) -> Result<()> {
        burn_token(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.token_account,
            ctx.accounts.token22_program.key(),
            &ctx.accounts.program_delegate,
        )?;

        close_mint(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // Currently rent collector is hardcoded to be the Program Delegaate
            &ctx.accounts.payer.to_account_info(),
            &ctx.accounts.program_delegate,
        )?;

        Ok(())
    }
}
