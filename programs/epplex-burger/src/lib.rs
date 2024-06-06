pub mod errors;
pub use errors::*;

pub mod state;
pub use state::*;

pub mod actions;
pub use actions::*;

pub mod id;
pub use id::ID;

pub mod utils;
pub use utils::*;

pub mod events;
pub use events::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    // ID as TOKEN_2022_PROGRAM_ID alternatively
    token_2022::{self, Token2022},
    token_interface::{Mint as MintInterface, TokenAccount as TokenAccountInterface},
};
use epplex_shared::{burn_token, close_mint, ADMINS};
// use wen_new_standard::accounts::TokenGroupMember;
use wen_new_standard::types::AddMetadataArgs;

#[derive(Clone)]
pub struct WenNewStandard;

impl Id for WenNewStandard {
    fn id() -> Pubkey {
        wen_new_standard::ID
    }
}

#[derive(Clone)]
pub struct WenRoyaltyDistribution;

impl Id for WenRoyaltyDistribution {
    fn id() -> Pubkey {
        wen_royalty_distribution::ID
    }
}

#[derive(Clone)]
pub struct TokenGroupMember2(wen_new_standard::accounts::TokenGroupMember);

impl anchor_lang::AccountDeserialize for TokenGroupMember2 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        // wen_new_standard::accounts::TokenGroupMember::deserialize(buf).map_err(|e| e.into())
        wen_new_standard::accounts::TokenGroupMember::deserialize(buf).map(TokenGroupMember2).map_err(|e| e.into())
        // wen_new_standard::accounts::TokenGroupMember::from_bytes(buf)
    }
}

impl anchor_lang::AccountSerialize for TokenGroupMember2 {}

impl anchor_lang::Owner for TokenGroupMember2 {
    fn owner() -> Pubkey {
        // pub use spl_token::ID is used at the top of the file
        ID
    }
}

// Implement the "std::ops::Deref" trait for better user experience
impl std::ops::Deref for TokenGroupMember2 {
    type Target = wen_new_standard::accounts::TokenGroupMember;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[program]
pub mod epplex_burger {
    use super::*;

    /*
     * WNS mints
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn wns_group_mint(ctx: Context<WnsGroupMint>, params: WnsGroupMintParams) -> Result<()> {
        WnsGroupMint::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn wns_member_mint(ctx: Context<WnsMemberMint>, params: WnsMemberMintParams) -> Result<()> {
        WnsMemberMint::actuate(ctx, params)
    }

    /*
     * Ephemeral context: Would have been better to simply CPI into Core during mint
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn ephemeral_rule_create(
        ctx: Context<EphemeralRuleCreate>,
        params: EphemeralRuleCreateParams,
    ) -> Result<()> {
        EphemeralRuleCreate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn ephemeral_data_add(
        ctx: Context<EphemeralDataAdd>,
        params: EphemeralDataAddParams,
    ) -> Result<()> {
        EphemeralDataAdd::actuate(ctx, params)
    }

    /*
     * Token Game Actions
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_burn(ctx: Context<TokenGameBurn>, params: TokenGameBurnParams) -> Result<()> {
        TokenGameBurn::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_vote(ctx: Context<TokenGameVote>, params: TokenGameVoteParams) -> Result<()> {
        TokenGameVote::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_reset(
        ctx: Context<TokenGameReset>,
        params: TokenGameResetParams,
    ) -> Result<()> {
        TokenGameReset::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_immunity(
        ctx: Context<TokenGameImmunity>,
        params: TokenGameImmunityParams,
    ) -> Result<()> {
        TokenGameImmunity::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_freeze(
        ctx: Context<TokenGameFreeze>,
        params: TokenGameFreezeParams,
    ) -> Result<()> {
        TokenGameFreeze::actuate(ctx, params)
    }

    /*
     * Game actions
     */
    #[access_control(ctx.accounts.validate(&ctx,))]
    pub fn game_create(ctx: Context<GameCreate>) -> Result<()> {
        GameCreate::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn game_start(ctx: Context<GameStart>, params: GameStartParams) -> Result<()> {
        GameStart::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn game_evaluate(ctx: Context<GameEvaluate>, params: GameEvaluateParams) -> Result<()> {
        GameEvaluate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn game_end(ctx: Context<GameEnd>, params: GameEndParams) -> Result<()> {
        GameEnd::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn game_update(ctx: Context<GameUpdate>, params: GameUpdateParams) -> Result<()> {
        GameUpdate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn game_close(ctx: Context<GameClose>) -> Result<()> {
        GameClose::actuate(ctx)
    }

    /*
     * Program Delegate
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_create(
        ctx: Context<ProgramDelegateCreate>,
        params: ProgramDelegateCreateParams,
    ) -> Result<()> {
        ProgramDelegateCreate::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_close(
        ctx: Context<ProgramDelegateClose>,
        params: ProgramDelegateCloseParams,
    ) -> Result<()> {
        ProgramDelegateClose::actuate(ctx, &params)
    }

    pub fn token_update(ctx: Context<TokenUpdate>, params: TokenUpdateParams) -> Result<()> {
        TokenUpdate::actuate(ctx, params)
    }

    pub fn token_thaw(ctx: Context<TokenThaw>, params: TokenThawParams) -> Result<()> {
        TokenThaw::actuate(ctx, params)
    }


    /*
     * DEPRECATED
     * Token actions
     */
    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn token_renew(
    //     ctx: Context<TokenRenew>,
    //     params: TokenRenewParams
    // ) -> Result<()> {
    //     TokenRenew::actuate(ctx, params)
    // }
    pub fn token_renew(_ctx: Context<TokenRenew>, _params: TokenRenewParams) -> Result<()> {
        // THIS SHOULD CPI INTO CORE
        Ok(())
    }

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn token_burn(
    //     ctx: Context<TokenBurn>,
    //     params: TokenBurnParams
    // ) -> Result<()> {
    //     TokenBurn::actuate(ctx, params)
    // }
    pub fn token_burn(ctx: Context<TokenBurn>, params: TokenBurnParams) -> Result<()> {
        // This is gated by payer and does not work on a WNS NFT anyway
        TokenBurn::actuate(ctx, params)
    }

    /*
     * DEPRECATED
     * Old collection stuff
     */
    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn whitelist_mint(
    //     ctx: Context<WhitelistMint>,
    //     params: WhitelistMintParams
    // ) -> Result<()> {
    //     WhitelistMint::actuate(ctx, params)
    // }
    pub fn whitelist_mint(
        _ctx: Context<WhitelistMint>,
        _params: WhitelistMintParams,
    ) -> Result<()> {
        Ok(())
    }

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn collection_mint(
    //     ctx: Context<CollectionMint>,
    //     params: CollectionMintParams,
    // ) -> Result<()> {
    //     CollectionMint::actuate(ctx, params)
    // }
    pub fn collection_mint(
        _ctx: Context<CollectionMint>,
        _params: CollectionMintParams,
    ) -> Result<()> {
        Ok(())
    }
}
