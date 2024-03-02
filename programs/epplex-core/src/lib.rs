pub mod actions;
pub mod id;
pub mod security;
pub mod state;
pub mod utils;
pub mod errors;

pub use actions::*;
pub use id::ID;
pub use state::*;
pub use utils::*;
pub use errors::*;

use anchor_lang::prelude::*;
use spl_token_2022::extension::ExtensionType;
use anchor_spl::associated_token::AssociatedToken;

#[program]
pub mod epplex_core {
    use super::*;

    /*
     * Does the mint account creation and mints it
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_mint(ctx: Context<TokenMint>, params: TokenCreateParams) -> Result<()> {
        TokenMint::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_mint(ctx: Context<CollectionMint>, params: TokenCollectionCreateParams) -> Result<()> {
        CollectionMint::actuate(ctx, params)
    }


    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_create(ctx: Context<CollectionCreate>, params: CollectionCreateParams) -> Result<()> {
        CollectionCreate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_close(ctx: Context<CollectionClose>, params: CollectionCloseParams) -> Result<()> {
        CollectionClose::actuate(ctx, params)
    }

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn collection_mint(ctx: Context<CollectionMint>, params: TokenCreateParams) -> Result<()> {
    //     CollectionMint::actuate(ctx, params)
    // }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn global_collection_config_create(ctx: Context<GlobalCollectionConfigCreate>) -> Result<()> {
        GlobalCollectionConfigCreate::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn global_collection_config_close(ctx: Context<GlobalCollectionConfigClose>) -> Result<()> {
        GlobalCollectionConfigClose::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn rule_create(
        ctx: Context<RuleManage>,
        seed: u64,
        rule_creator: Pubkey,
        renewal_price: u64,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts.create_rule(seed, rule_creator, renewal_price, treasury)
    }

    pub fn rule_modify(
        ctx: Context<RuleManage>,
        seed: u64,
        rule_creator: Pubkey,
        renewal_price: u64,
        treasury: Pubkey,
    ) -> Result<()> {
        ctx.accounts.modify_rule(seed, rule_creator, renewal_price, treasury)
    }

    pub fn membership_create(
        ctx: Context<MembershipCreate>,
        time: i64,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create(time, name, symbol, uri, ctx.bumps)
    }

    pub fn membership_burn(
        ctx: Context<MembershipBurn>,
    ) -> Result<()> {
        ctx.accounts.burn(ctx.bumps)
    }

    pub fn time_add(
        ctx: Context<TimeManage>,
        time: u64,
    ) -> Result<()> {
        ctx.accounts.add(time)
    }

    pub fn time_remove(
        ctx: Context<TimeManage>,
        time: u64,
    ) -> Result<()> {
        ctx.accounts.remove(time)
    }


}
