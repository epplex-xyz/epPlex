pub mod actions;
pub mod errors;
pub mod id;
pub mod security;
pub mod state;
pub mod utils;

pub use actions::*;
pub use errors::*;
pub use id::ID;
pub use state::*;
pub use utils::*;

use anchor_lang::solana_program;

use anchor_spl::{
    associated_token::{create, Create},
    token_2022::spl_token_2022,
    token_2022::spl_token_2022::{
        extension::ExtensionType,
        extension::{
            metadata_pointer::instruction::initialize as initialize_metadata_pointer,
            transfer_hook::instruction::initialize as intialize_transfer_hook,
        },
        instruction::{
            initialize_mint2, initialize_mint_close_authority, initialize_permanent_delegate,
            AuthorityType
        },
    },
    associated_token::AssociatedToken,
    token_interface::{
        Token2022,
        spl_token_metadata_interface::state::Field,
        spl_token_metadata_interface::state::TokenMetadata,
        spl_token_metadata_interface::instruction::initialize as initialize_metadata_account,
        {mint_to, set_authority, MintTo}
    }
};


#[program]
pub mod epplex_core {
    use super::*;

    /*
     * Ephemeral membership
     */
    pub fn rule_create(ctx: Context<RuleManage>, params: RuleManageParams) -> Result<()> {
        RuleManage::rule_create(ctx, params)
    }

    pub fn rule_modify(ctx: Context<RuleManage>, params: RuleManageParams) -> Result<()> {
        RuleManage::rule_modify(ctx, params)
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

    pub fn membership_append(ctx: Context<MembershipAppend>, time: i64) -> Result<()> {
        ctx.accounts.append(time, ctx.bumps)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn membership_burn(ctx: Context<MembershipBurn>) -> Result<()> {
        MembershipBurn::burn(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn membership_wns_burn(ctx: Context<MembershipWnsBurn>) -> Result<()> {
        MembershipWnsBurn::burn(ctx)
    }

    pub fn time_add(ctx: Context<TimeManage>, time: u64) -> Result<()> {
        ctx.accounts.add(time)
    }

    pub fn time_remove(ctx: Context<TimeManage>, time: u64) -> Result<()> {
        ctx.accounts.remove(time)
    }

    /*
     * DEPRECATED
     * Create mint account and mints to owner
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_mint(ctx: Context<TokenMint>, params: TokenCreateParams) -> Result<()> {
        TokenMint::actuate(ctx, params)
    }

    /*
     * DEPRECATED
     * Collection stuff
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_mint(
        ctx: Context<CollectionMint>,
        params: TokenCollectionCreateParams,
    ) -> Result<()> {
        CollectionMint::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_create(
        ctx: Context<CollectionCreate>,
        params: CollectionCreateParams,
    ) -> Result<()> {
        CollectionCreate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_close(
        ctx: Context<CollectionClose>,
        params: CollectionCloseParams,
    ) -> Result<()> {
        CollectionClose::actuate(ctx, params)
    }

    /*
     * DEPRECATED
     * Global collection stuff
     */
    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn global_collection_config_create(
        ctx: Context<GlobalCollectionConfigCreate>,
    ) -> Result<()> {
        GlobalCollectionConfigCreate::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn global_collection_config_close(ctx: Context<GlobalCollectionConfigClose>) -> Result<()> {
        GlobalCollectionConfigClose::actuate(ctx)
    }
}
