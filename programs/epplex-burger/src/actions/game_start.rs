use crate::*;

#[derive(Accounts)]
#[instruction(params: GameStartParams)]
pub struct GameStart<'info> {
    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        signer,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: SystemAccount<'info>,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameStartParams {
    pub end_timestamp: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String,
    pub is_encrypted: bool,
    pub public_encrypt_key: String
}

impl GameStartParams {
    pub fn validate_params(&self) -> Result<()> {
        // Fail if timestamp is not in the future
        if !(Clock::get().unwrap().unix_timestamp < self.end_timestamp) {
            return err!(BurgerError::InvalidGameDuration);
        };

        // Public encrypt key cannot be empty
        if self.is_encrypted {
            if self.public_encrypt_key.is_empty() {
                return err!(BurgerError::RequiresEncryption)
            }
        }


        if self.vote_type.eq(&VoteType::None)
            || self.input_type.eq(&InputType::None)
            || self.game_prompt.is_empty()
        {
            return err!(BurgerError::InvalidStartParams);
        }

        Ok(())
    }
}

impl GameStart<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &GameStartParams) -> Result<()> {
        params.validate_params()?;

        self.game_config.can_start_game()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: GameStartParams) -> Result<()> {
        ctx.accounts.game_config.start(params)
    }
}
