use anchor_lang::prelude::*;
use ephemerality::{cpi::accounts::CollectionCreate, program::Ephemerality};
use ephemerality::CollectionCreateParams;

declare_id!("DWQ12BSvpNq6AxX18Xgm72avoCT8nL8G7R886NeiLFeN");

#[program]
pub mod ep_mint {
    use super::*;
    
    pub fn init_mint_pool(ctx: Context<InitMintPool>, name: String) -> Result<()> {

        //init mint_pool
        let mint_pool = &mut ctx.accounts.mint_pool;
        mint_pool.authority = ctx.accounts.creator.key();
        mint_pool.items =  Vec::new();
        mint_pool.items_minted = 0;
        mint_pool.bump = *ctx.bumps.get("mint_pool").unwrap();


        //create cpi
        let cpi_program = ctx.accounts.epplex_program.to_account_info();

        let cpi_accounts = CollectionCreate {
            //mint: ctx.accounts.collection_mint.to_account_info(),
            program_delegate: ctx.accounts.program_delegate.to_account_info(),
            collection_config: ctx.accounts.collection_config.to_account_info(),
            token22_program: ctx.accounts.token22_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            payer: ctx.accounts.creator.to_account_info()
        };

        //TODO configure this off-chain
        let params = CollectionCreateParams {
            authority: mint_pool.key(),
            renewal_price: 100,
            standard_duration: 100,
            grace_period: 100,
            treasury: mint_pool.key(),
            collection_size: 10000,
            collection_name: name
        };


        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        ephemerality::cpi::create_collection(cpi_ctx, params)?;
        
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitMintPool<'info> {

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + MintPool::MAX_SIZE, seeds = [b"pool", name.as_bytes()], bump
    )]
    pub mint_pool: Account<'info, MintPool>,

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
pub struct MintPool {
    pub authority: Pubkey,
    pub items: Vec<Pubkey>,
    pub items_minted: u32,
    pub bump: u8
}

impl MintPool {
    pub const MAX_SIZE: usize = 32 + 32 + 32 + 1;
}

#[derive(Clone)]
pub struct Token2022;

impl Id for Token2022 {
    fn id() -> Pubkey {
        spl_token_2022::ID
    }
}
