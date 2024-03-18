use crate::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::Token2022};
use epplex_core::program::EpplexCore;
use epplex_core::{
    EphemeralData, EphemeralRule, SEED_EPHEMERAL_AUTH, SEED_EPHEMERAL_DATA, SEED_EPHEMERAL_RULE,
};

#[derive(Accounts)]
#[instruction(params: TokenGameBurnParams)]
pub struct TokenGameBurn<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    // this will be created
    #[account(mut)]
    /// CHECK: cpi checks
    pub token_account: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = mint.key(),
        token::token_program = token22_program.key(),
    )]
    pub source_token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(
        seeds = [
            SEED_GAME_CONFIG
        ],
        bump = game_config.bump,
        constraint = game_config.token_group == group_member.group @ BurgerError::CollectionInvalid,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(
        seeds = [
            wen_new_standard::MEMBER_ACCOUNT_SEED,
            mint.key().as_ref()
        ],
        seeds::program = wen_new_standard::ID.key(),
        constraint = mint.key() == group_member.mint @ BurgerError::IncorrectMint,
        bump,
    )]
    pub group_member: Account<'info, wen_new_standard::TokenGroupMember>,

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,

    /**
     * Epplex Core accounts
     */
    #[account(
        seeds = [
            SEED_EPHEMERAL_RULE,
            rule.seed.to_le_bytes().as_ref()
        ],
        seeds::program = epplex_core.key(),
        bump,
    )]
    pub rule: Account<'info, EphemeralRule>,

    #[account(
        mut,
        seeds = [
            SEED_EPHEMERAL_DATA,
            mint.key().as_ref()
        ],
        seeds::program = epplex_core.key(),
        bump,
    )]
    pub data: Account<'info, EphemeralData>,

    #[account(
        mut,
        seeds = [
            SEED_EPHEMERAL_AUTH
        ],
        seeds::program = epplex_core.key(),
        bump
    )]
    /// CHECK:
    pub epplex_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub epplex_treasury: SystemAccount<'info>,

    pub epplex_core: Program<'info, EpplexCore>,

    /*
       WNS stuff
    */
    // Transfer Hook Accounts
    #[account()]
    /// CHECK: no need to check it out, the invoke_transfer will check for us
    pub metas_account_list: AccountInfo<'info>,

    // For burning
    #[account(
        seeds = [wen_new_standard::MANAGER_SEED],
        seeds::program = wen_new_standard::ID,
        bump
    )]
    pub manager: Account<'info, wen_new_standard::Manager>,

    // Approve
    #[account(mut)]
    /// CHECK: initialized token account or unitialized token account, checks in cpi
    pub approve_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: initialized token account or unitialized token account, checks in cpi
    pub distribution_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: cpi checks
    pub distribution_account: UncheckedAccount<'info>,
    #[account()]
    /// CHECK: This account can be any mint or SOL
    pub payment_mint: UncheckedAccount<'info>,

    pub wrd: Program<'info, WenRoyaltyDistribution>,
    pub wns: Program<'info, WenNewStandard>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameBurnParams {}

impl TokenGameBurn<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenGameBurnParams) -> Result<()> {
        // TODO: need to check for immunity
        self.game_config.can_evaluate()
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenGameBurnParams) -> Result<()> {
        // Closes the data pda as well
        epplex_core::cpi::membership_wns_burn(CpiContext::new(
            ctx.accounts.epplex_core.to_account_info(),
            epplex_core::cpi::accounts::MembershipWnsBurn {
                membership: ctx.accounts.mint.to_account_info(),
                source_ata: ctx.accounts.source_token_account.to_account_info(),
                membership_ata: ctx.accounts.token_account.to_account_info(),
                burner: ctx.accounts.payer.to_account_info(),
                epplex_treasury: ctx.accounts.epplex_treasury.to_account_info(), // update_auth = perm_delegate
                rule: ctx.accounts.rule.to_account_info(),
                data: ctx.accounts.data.to_account_info(),
                epplex_authority: ctx.accounts.epplex_authority.to_account_info(),
                token22_program: ctx.accounts.token22_program.to_account_info(),

                metas_account_list: ctx.accounts.metas_account_list.to_account_info(),
                manager: ctx.accounts.manager.to_account_info(),
                approve_account: ctx.accounts.approve_account.to_account_info(),
                distribution_token_account: ctx
                    .accounts
                    .distribution_token_account
                    .to_account_info(),
                distribution_account: ctx.accounts.distribution_account.to_account_info(),
                payment_mint: ctx.accounts.payment_mint.to_account_info(),
                wrd: ctx.accounts.wrd.to_account_info(),
                wns: ctx.accounts.wns.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        ))?;

        ctx.accounts.game_config.bump_burn_amount()?;

        emit!(EvTokenGameBurn {
            game_round_id: ctx.accounts.game_config.game_round,
            nft: ctx.accounts.mint.key(),
            participant: epplex_shared::get_token_account_owner(
                &ctx.accounts.token_account.to_account_info()
            )?,
            burn_timestamp: Clock::get().unwrap().unix_timestamp,
        });

        Ok(())
    }
}
