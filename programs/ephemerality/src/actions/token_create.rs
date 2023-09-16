use crate::*;
use anchor_spl::token_2022::{
    Token2022,
    spl_token_2022,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenAccount(spl_token_2022::state::Account);

impl anchor_lang::AccountDeserialize for TokenAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Account>::unpack(
            buf,
        )
            .map(|t| TokenAccount(t.base))
            .map_err(Into::into)
    }
}

impl anchor_lang::AccountSerialize for TokenAccount {}


impl anchor_lang::Owner for TokenAccount {
    fn owner() -> Pubkey {
        spl_token_2022::ID
    }
}


#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump,
    )]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {}

impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &TokenCreateParams) -> Result<()> {

        Self::add_closing_authority(
            &ctx.accounts.mint,
            ctx.accounts.token22_program.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.program_delegate.key(),
        )?;
        // Self::add_permanent_delegate(ctx)?;


        // Initialize mint
        let ix = spl_token_2022::instruction::initialize_mint(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.payer.key(), // this could be different I guess
            Some(&ctx.accounts.payer.key()),
            0, // NFTs have 0 decimals
        )?;

        let account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.rent.to_account_info()
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;


        Ok(())
    }

    fn add_permanent_delegate(mintAccount: AccountInfo, program: Pubkey, mint: Pubkey, program_delegate: Pubkey) -> Result<()> {
        let account_infos: Vec<AccountInfo> = vec![
            mintAccount.to_account_info()
        ];

        // System::
        let ix = spl_token_2022::instruction::initialize_permanent_delegate(
            &program,
            &mint,
            &program_delegate,
        )?;

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }

    fn add_closing_authority(mintAccount: &AccountInfo, program: Pubkey, mint: Pubkey, program_delegate: Pubkey) -> Result<()> {
        let account_infos: Vec<AccountInfo> = vec![
            mintAccount.to_account_info(),
        ];

        let ix = spl_token_2022::instruction::initialize_mint_close_authority(
            &program,
            &mint,
            Some(&program_delegate),
        )?;

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }
}
