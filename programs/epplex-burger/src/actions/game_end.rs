use anchor_lang::prelude::*;

use crate::{ BurgerError, GameConfig, GamePhase };

#[derive(Accounts)]
pub struct GameEnd<'info> {
    pub payer: Signer<'info>,

    // we expect this to be already initialized
    pub game_config: Account<'info, GameConfig>,

    pub system_program: Program<'info, System>,
}

impl GameEnd<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        // ? if in last stage err
        let phase = self.game_config.game_phase;

        // ? assuming you cannot change game phase when in the last stage
        if phase == GamePhase::Elimination {
            return err!(BurgerError::GamePhaseLastStage);
        }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        game_config.game_phase = GamePhase::Elimination;

        Ok(())
    }
}
