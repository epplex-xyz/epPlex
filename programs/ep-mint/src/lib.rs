use anchor_lang::prelude::*;
use ephemerality::{cpi::accounts::CollectionCreate, cpi::accounts::TokenCreate, program::Ephemerality};
use ephemerality::{CollectionCreateParams, CollectionConfig, TokenCreateParams};
use spl_token_2022;
declare_id!("DWQ12BSvpNq6AxX18Xgm72avoCT8nL8G7R886NeiLFeN");

const GUARD_SEED: &[u8] = b"guard";

#[program]
pub mod ep_mint {

    use super::*;
    
    pub fn init_mint_guard(ctx: Context<InitMintGuard>, params: InitMintGuardParams) -> Result<()> {

        //init mint pool
        let mint_guard = &mut ctx.accounts.mint_guard;
        mint_guard.authority = ctx.accounts.creator.key();
        mint_guard.items_minted = 0;
        mint_guard.bump = *ctx.bumps.get("mint_guard").unwrap();

        //create cpi
        let cpi_program = ctx.accounts.epplex_program.to_account_info();

        let cpi_accounts = CollectionCreate {
            mint: ctx.accounts.collection_mint.to_account_info(),
            program_delegate: ctx.accounts.program_delegate.to_account_info(),
            collection_config: ctx.accounts.collection_config.to_account_info(),
            token22_program: ctx.accounts.token22_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            payer: ctx.accounts.creator.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        //create params
        let collection_create_params = CollectionCreateParams {
            authority: mint_guard.key(),
            renewal_price: params.collection_renewal_price,
            standard_duration: params.collection_standard_duration,
            grace_period: params.collection_grace_period,
            treasury: mint_guard.key(),
            collection_size: params.collection_size,
            collection_name: params.collection_name
        };

        ephemerality::cpi::create_collection(cpi_ctx, collection_create_params)?;
        
        Ok(())
    }

    pub fn mint_from_collection(ctx: Context<MintFromCollection>, collection_name: String) -> Result<()> {
        
        let collection_config = &mut ctx.accounts.collection_config;
        let mint_guard = &mut ctx.accounts.mint_guard;

        //create cpi
        let cpi_program = ctx.accounts.epplex_program.to_account_info();

        let cpi_accounts = TokenCreate {
            mint: ctx.accounts.token_mint.to_account_info(),
            program_delegate: ctx.accounts.program_delegate.to_account_info(),
            payer: ctx.accounts.minter.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            token22_program: ctx.accounts.token22_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        //create token creation params
        let mut token_name = collection_config.collection_name.to_owned();
        token_name.push_str(&mint_guard.items_minted.to_string());

        let params = TokenCreateParams {
            destroy_timestamp_offset: 1000,
            name: token_name,
            //TODO add collection symbol to collection config
            symbol: "BRGR".to_string(),
            uri: "".to_string()
        };

        ephemerality::cpi::token_create(cpi_ctx, params)?;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(params: InitMintGuardParams)]
pub struct InitMintGuard<'info> {

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + MintGuard::MAX_SIZE, seeds = [GUARD_SEED, params.collection_name.as_bytes()], bump
    )]
    pub mint_guard: Account<'info, MintGuard>,

    pub epplex_program: Program<'info, Ephemerality>,
    
    #[account(mut)]
    /// CHECK
    pub collection_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub collection_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,
    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct MintGuard {
    pub authority: Pubkey,
    pub items_minted: u32,
    pub bump: u8
}

impl MintGuard {
    pub const MAX_SIZE: usize = 32 + 4 + 1;
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitMintGuardParams {
    collection_renewal_price: u64,
    collection_standard_duration: u32,
    collection_grace_period: i64,
    collection_size: u32,
    collection_name: String
}

#[derive(Accounts)]
#[instruction(collection_name: String)]
pub struct MintFromCollection<'info> {

    #[account(mut)]
    pub minter: Signer<'info>,

    #[account(
        seeds = [GUARD_SEED, collection_name.as_bytes()],
        bump = mint_guard.bump
    )]
    pub mint_guard: Account<'info, MintGuard>,

    pub epplex_program: Program<'info, Ephemerality>,

    #[account()]
    /// CHECK
    pub collection_config: Account<'info, CollectionConfig>,

    /// CHECK
    #[account(mut)]
    pub token_mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>
}


#[derive(Clone)]
pub struct Token2022;

impl Id for Token2022 {
    fn id() -> Pubkey {
        spl_token_2022::ID
    }
}