use crate::*;

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

    #[account(
        mut,
        token::mint = mint.key(),
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(
        mut,
        close = payer,
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        seeds = [
            SEED_GAME_CONFIG
        ],
        bump = game_config.bump,
        constraint = group_member.group == game_config.group_pda,
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
        bump,
    )]
    pub group_member: Account<'info, MyTokenGroupMember>,

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
        close = epplex_treasury,
        seeds = [
            SEED_EPHEMERAL_DATA,
            mint.key().as_ref()
        ],
        seeds::program = epplex_core.key(),
        bump,
    )]
    pub data: Account<'info, EphemeralData>,

    #[account(
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
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameBurnParams {}

impl TokenGameBurn<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenGameBurnParams) -> Result<()> {
        // TODO: need to check for immunity
        self.game_config.can_evaluate()
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenGameBurnParams) -> Result<()> {
        epplex_core::cpi::membership_burn(CpiContext::new(
            ctx.accounts.epplex_core.to_account_info(),
            epplex_core::cpi::accounts::MembershipBurn {
                membership: ctx.accounts.mint.to_account_info(),
                membership_ata: ctx.accounts.token_account.to_account_info(),
                burner: ctx.accounts.payer.to_account_info(),
                epplex_treasury: ctx.accounts.epplex_treasury.to_account_info(), // update_auth = perm_delegate
                rule: ctx.accounts.rule.to_account_info(),
                data: ctx.accounts.token22_program.to_account_info(),
                epplex_authority: ctx.accounts.epplex_authority.to_account_info(),
                token22_program: ctx.accounts.token22_program.to_account_info(),
            },
        ))?;

        ctx.accounts.game_config.bump_burn_amount()
    }
}
