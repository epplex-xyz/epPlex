use anchor_lang::prelude::borsh::BorshDeserialize;
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenBurnParams)]
pub struct TokenBurn<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    // TODO check that this is in fact a token account for th emint
    #[account(
        mut
    )]
    /// CHECK
    pub token_account: AccountInfo<'info>,

    pub token22_program: Program<'info, Token2022>,
}

// TODO how to deserialize and read the data?
// maybe should look through andy capture the flag video
// maybe could also try upgrading anchor cli
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBurnParams {}

impl TokenBurn<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        _params: &TokenBurnParams,
    ) -> Result<()> {
        msg!("Here");
        let data_bytes = ctx.accounts.mint.try_borrow_data()?;
        let (header_bytes, rest) = data_bytes.split_at(374);
        msg!("Rest {:?}", rest);
        let metadata: Metadata = Metadata::try_from_slice(rest)?;

        // let data_ref = &mut  data.as_ref();
        // let test: Mint22 = ctx.accounts.mint.try_into(Mint22)?;
        msg!("Mint data {:?}", metadata);
        // let mint22: Mint22 = BorshDeserialize::deserialize(data)?;

        // let mint_data = ctx.accounts.mint.clone();
        // msg!("Test {:?}", mint_data.destroy_timestamp_value);

        // AnchorDeserialize::deserialize(&data[406..])?;
        // let mint_data: Mint22 = Mint22::try_from_slice(&data)?;
        // let mint_data: Metadata = Metadata::try_from_slice(
        //     &data[406..]
        //         // .try_into()
        //         // .map_err(|_| ProgramError::InvalidArgument)?,
        // )?;
        let destroy_timestamp = metadata.destroy_timestamp_value.parse::<i64>().unwrap();
        let now = Clock::get().unwrap().unix_timestamp;
        if now < destroy_timestamp {
            return err!(EphemeralityError::DestroyTimestampNotExceeded);
        }

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
            &ctx.accounts.program_delegate.to_account_info(),
            &ctx.accounts.program_delegate,
        )?;

        Ok(())
    }
}
