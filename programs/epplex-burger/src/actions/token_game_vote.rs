use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenGameVoteParams)]
pub struct TokenGameVote<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = payer,
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>, // Used to verify owner

    #[account(
        seeds = [
            wen_new_standard::MEMBER_ACCOUNT_SEED,
            mint.key().as_ref()
        ],
        seeds::program = wen_new_standard::ID,
        bump,
    )]
    pub group_member: Account<'info, wen_new_standard::TokenGroupMember>,

    #[account(
        seeds = [
            SEED_GAME_CONFIG
        ],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = update_authority.bump
    )]
    pub update_authority: Account<'info, ProgramDelegate>,

    pub token22_program: Program<'info, Token2022>,

    // WNS programs
    #[account(
        seeds = [
            wen_new_standard::MANAGER_SEED
        ],
        seeds::program = wen_new_standard::ID,
        bump
    )]
    pub manager: Account<'info, wen_new_standard::Manager>,
    pub wns: Program<'info, WenNewStandard>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameVoteParams {
    pub message: String,
}

impl TokenGameVote<'_> {
    pub fn validate(&self, ctx: &Context<Self>, params: &TokenGameVoteParams) -> Result<()> {
        self.game_config
            .check_valid_collection(&self.group_member, self.mint.key())?;

        self.game_config.validate_input(&params.message)?;

        // Check vote_many and vote_once
        let game_state = fetch_metadata_field(GAME_STATE, &ctx.accounts.mint.to_account_info())?;
        self.game_config.check_vote_eligibility(game_state)?;

        // Check that the game is in progress
        self.game_config
            .assert_game_status(GameStatus::InProgress)?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenGameVoteParams) -> Result<()> {
        let game_state = fetch_metadata_field(GAME_STATE, &ctx.accounts.mint.to_account_info())?;
        ctx.accounts
            .game_config
            .bump_submission_amount(game_state)?;

        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];
        // Update game state
        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Key(GAME_STATE.to_string()),
            params.message.clone(),
        )?;

        // Record voting timestamp
        let now = Clock::get().unwrap().unix_timestamp;
        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Key(VOTING_TIMESTAMP.to_string()),
            now.to_string(),
        )?;

        if ctx.accounts.game_config.vote_type.eq(&VoteType::VoteOnce) {
            // TODO this needs to be refactored out
            anchor_spl::token_interface::approve(
                CpiContext::new(
                    ctx.accounts.token22_program.to_account_info(),
                    anchor_spl::token_interface::Approve {
                        to: ctx.accounts.token_account.to_account_info(),
                        delegate: ctx.accounts.update_authority.to_account_info(),
                        authority: ctx.accounts.payer.to_account_info(),
                    },
                ),
                1
            )?;

            wen_new_standard::cpi::freeze_mint_account(
                CpiContext::new_with_signer(
                    ctx.accounts.wns.to_account_info(),
                    wen_new_standard::cpi::accounts::FreezeDelegatedAccount {
                        payer: ctx.accounts.payer.to_account_info(),
                        user: ctx.accounts.payer.to_account_info(),
                        delegate_authority: ctx.accounts.update_authority.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                        mint_token_account: ctx.accounts.token_account.to_account_info(),
                        manager: ctx.accounts.manager.to_account_info(),
                        token_program: ctx.accounts.token22_program.to_account_info(),
                    },
                    &[&seeds[..]],
                )
            )?;
        }

        emit!(EvTokenGameVote {
            participant: ctx.accounts.payer.key(),
            answer: params.message,
            game_round_id: ctx.accounts.game_config.game_round,
            nft: ctx.accounts.mint.key(),
            vote_timestamp: Clock::get().unwrap().unix_timestamp,
        });

        Ok(())
    }
}
