use anchor_lang::prelude::*;

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
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        self.game_config.check_phase_end_ts()?;

        // ! make sure that the metadata fields are populated 

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        game_config.game_status = GameStatus::Finished;

        Ok(())
    }
}
