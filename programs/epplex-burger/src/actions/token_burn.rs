use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use epplex_shared::Token2022;
use solana_program::program_pack::Pack;

#[derive(Accounts)]
#[instruction(params: TokenBurnParams)]
pub struct TokenBurn<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        token::mint = mint.key(),
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(
        mut,
        close = payer,
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Option<Account<'info, GameConfig>>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBurnParams {}

impl TokenBurn<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        _params: &TokenBurnParams,
    ) -> Result<()> {
        // TODO: need to check for immunity
        // this burn function has too much responsibility

        match &self.game_config {
            Some(game_config) =>
                game_config.assert_game_status(GameStatus::Finished)?,
            None =>
                check_has_expired(&ctx.accounts.mint.to_account_info())?,
        }

        // if self.game_config.game_status.ne(&GameStatus::Finished) {
        //     check_has_expired(&ctx.accounts.mint.to_account_info())?;
        // }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenBurnParams) -> Result<()> {
        burn_token(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.token_account.to_account_info(),
            ctx.accounts.token22_program.key(),
            &ctx.accounts.permanent_delegate.to_account_info(),
        )?;

        // Close mint account
        close_mint(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // Currently rent collector is hardcoded to be the Program Delegaate
            &ctx.accounts.payer.to_account_info(),
            // Authority to close the mint
            &ctx.accounts.permanent_delegate.to_account_info(),
        )?;

        // Can only close the ATA if we are the owners
        let token_account = ctx.accounts.token_account.to_account_info();
        let state = spl_token_2022::state::Account::unpack_from_slice(
            &token_account.try_borrow_data()?
        )?;
        if state.owner == ctx.accounts.payer.key() {
            anchor_spl::token_interface::close_account(
                CpiContext::new(
                    ctx.accounts.token22_program.to_account_info(),
                    anchor_spl::token_interface::CloseAccount {
                        account: ctx.accounts.token_account.to_account_info().clone(),
                        destination: ctx.accounts.payer.to_account_info().clone(),
                        authority: ctx.accounts.payer.to_account_info().clone(),
                    },
                ),
            )?;
        }

        // Another one bites the dust
        match &mut ctx.accounts.game_config {
            Some(game_config) => game_config.bump_burn_amount()?,
            None => (),
        };

        Ok(())
    }
}
