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

    pub system_program: Program<'info, System>,
}

impl GameEnd<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        self.game_config.check_game_ended()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        ctx.accounts.game_config.end()
    }
}
