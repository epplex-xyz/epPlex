use crate::*;

use anchor_lang::prelude::*;
use epplex_shared::Token2022;

use crate::{GameConfig, GameStatus, SEED_GAME_CONFIG};

#[derive(Accounts)]
pub struct GameEnd<'info> {
    #[account(address = game_config.game_master)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        mut,
        mint::token_program = token22_program.key(), // ? is this check necessary
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl GameEnd<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        self.game_config.check_phase_end_ts()?;

        // ! make sure that the metadata fields are populated.
        // ! meaning the account participated in the game
        self.game_config
            .check_metadata_fields_filled(&ctx.accounts.mint.to_account_info())?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        game_config.game_status = GameStatus::Finished;

        Ok(())
    }
}
