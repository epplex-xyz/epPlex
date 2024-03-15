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
    pub game_name: String,
    pub is_encrypted: bool,
    pub public_encrypt_key: String,
    pub rule_seed: u64,
    pub token_group: Pubkey,
}

impl GameStartParams {
    pub fn validate_params(&self) -> Result<()> {
        // Fail if timestamp is not in the future
        if Clock::get().unwrap().unix_timestamp >= self.end_timestamp {
            return err!(BurgerError::IncorrectEndtime);
        };

        // Public encrypt key cannot be empty
        if self.is_encrypted && self.public_encrypt_key.is_empty() {
            return err!(BurgerError::RequiresEncryption);
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
        self.game_config.can_start_game()
    }

    pub fn actuate(ctx: Context<Self>, params: GameStartParams) -> Result<()> {
        ctx.accounts.game_config.start(params)?;

        emit!(EvGameStart {
            game_round_id: ctx.accounts.game_config.game_round,
            game_start_timestamp: Clock::get().unwrap().unix_timestamp,
            game_end_timestamp: ctx.accounts.game_config.phase_end_timestamp,
            game_name: ctx.accounts.game_config.game_name.clone(),
            game_prompt: ctx.accounts.game_config.game_prompt.clone(),
            input_type: ctx.accounts.game_config.input_type,
            vote_type: ctx.accounts.game_config.vote_type,
            public_encrypt_key: ctx.accounts.game_config.public_encrypt_key.clone(),
            burn_amount: ctx.accounts.game_config.burn_amount,
            submission_amount: ctx.accounts.game_config.submission_amount,
        });

        Ok(())
    }
}
