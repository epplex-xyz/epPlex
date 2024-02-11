use anchor_lang::prelude::*;

use crate::{ BurgerError, GameConfig, GamePhase, ADMIN_PUBKEY };

#[derive(Accounts)]
pub struct GameTransition<'info> {
    #[account(address = ADMIN_PUBKEY)]
    pub payer: Signer<'info>,

    // we expect this acc to already be initialized
    pub game_config: Account<'info, GameConfig>,

    pub system_program: Program<'info, System>,
}

impl GameTransition<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        // ? if in last stage err
        let phase = self.game_config.game_phase;

        // ? assuming you cannot transition in the last phase
        if phase == GamePhase::Elimination {
            return err!(BurgerError::GamePhaseLastStage);
        }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        // ? when we change phase, do we also reset the timestamps

        // move to the next phase
        match game_config.game_phase {
            GamePhase::None => {
                game_config.game_phase = GamePhase::Announcement;
            }
            GamePhase::Announcement => {
                game_config.game_phase = GamePhase::Voting;
            }
            GamePhase::Voting => {
                game_config.game_phase = GamePhase::Elimination;
            }
            // checked in validate so this arm won't ever run
            GamePhase::Elimination => {
                game_config.game_phase = GamePhase::Elimination;
            }
        }

        Ok(())
    }
}
