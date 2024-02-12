use anchor_lang::prelude::*;

use crate::{ GameConfig, GamePhase };

#[derive(Accounts)]
pub struct GameTransition<'info> {
    #[account(address = game_config.game_master)]
    pub payer: Signer<'info>,

    // we expect this acc to already be initialized
    pub game_config: Account<'info, GameConfig>,

    pub system_program: Program<'info, System>,
}

impl GameTransition<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        // make sure that the game hasn't ended
        self.game_config.check_game_ended()?;

        // make sure that invalid timestamps aren't passed in
        self.game_config.check_duration()?;

        self.game_config.check_phase_end_ts()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, phase_end: i64) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        let now = Clock::get().unwrap().unix_timestamp;

        // let duration = game_config.phase_end - game_config.phase_start;

        // todo (Jimii): create a `reset_ts` associated helper function
        match game_config.game_phase {
            GamePhase::None => {
                game_config.game_phase = GamePhase::Announcement;

                // reset timestamps
                game_config.phase_start = now;
                game_config.phase_end = phase_end;
            }
            GamePhase::Announcement => {
                game_config.game_phase = GamePhase::Voting;

                // reset timestamps
                game_config.phase_start = now;
                game_config.phase_end = phase_end;
            }
            GamePhase::Voting => {
                game_config.game_phase = GamePhase::Elimination;

                // reset timestamps
                game_config.phase_start = now;
                game_config.phase_end = phase_end;
            }
            // checked in validate so this arm won't ever run
            GamePhase::Elimination => {
                game_config.game_phase = GamePhase::Elimination;

                // reset timestamps
                game_config.phase_start = now;
                game_config.phase_end = phase_end;
            }
        }

        Ok(())
    }
}
