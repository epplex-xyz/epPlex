use anchor_spl::token_interface::MintTo;
use ephemeral_metadata::program::EphemeralMetadata;
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct MintTokenFromCollection<'info> {
    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub ata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_metadata: UncheckedAccount<'info>,
    
    #[account()]
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(mut)]
    ///CHECK: Checked in validate
    pub treasury: AccountInfo<'info>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint_authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, EphemeralMetadata>
}

impl MintTokenFromCollection<'_> {
    pub fn validate(&self, ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {

        if ctx.accounts.mint_authority.key() != ctx.accounts.collection_config.authority {
            return err!(MintError::UnauthorizedMintAuthority)
        };

        if ctx.accounts.treasury.key() != ctx.accounts.collection_config.treasury {
            return err!(MintError::InvalidTreasuryAccount)
        };

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenCreateParams) -> Result<()> {

        //transfer mint price to treasury
        transfer_sol(
            &ctx.accounts.system_program,
            &ctx.accounts.payer,
            &ctx.accounts.treasury,
            ctx.accounts.collection_config.mint_price
        )?;

        // Create the ephemeral token
        TokenCreate::execute(
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.program_delegate.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.rent.to_account_info().clone(),
            ctx.accounts.token22_program.to_account_info().clone(),
            &[ExtensionType::GroupPointer, ExtensionType::GroupMemberPointer]
        )?;



        // Point Group Pointer to collection config
        Self::add_group_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.program_delegate.key(),
            ctx.accounts.collection_config.key()
        )?;

        // Create token metadata account
        create_metadata_account(
            ctx.accounts.metadata_program.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.token_metadata.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            params
        )?;

        // Point group Member Pointer to token metadata
        Self::add_group_member_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.program_delegate.key(),
            ctx.accounts.token_metadata.key()
        )?;

        // Create ATA
        anchor_spl::associated_token::create(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: ctx.accounts.payer.to_account_info(), // payer
                    associated_token: ctx.accounts.ata.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(), // owner
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                }
            ),
        )?;

        // Mint to ATA
        anchor_spl::token_interface::mint_to(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info().clone(),
                    to: ctx.accounts.ata.to_account_info().clone(),
                    authority: ctx.accounts.payer.to_account_info(),
                }
            ),
            1
        )?;

        Ok(())
    }

    pub fn add_group_pointer(
        token_program_id: Pubkey,
        mint_account: &AccountInfo,
        authority: Pubkey,
        group_address: Pubkey,
    ) -> Result<()> {
        let ix = spl_token_2022::extension::group_pointer::instruction::initialize(
            &token_program_id,
            &mint_account.key(),
            Some(authority),
            Some(group_address)
        )?;
    
        let account_infos: Vec<AccountInfo> = vec![
            mint_account.to_account_info(),
        ];
    
        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;
    
        Ok(())
    }

    pub fn add_group_member_pointer(
        token_program_id: Pubkey,
        mint_account: &AccountInfo,
        authority: Pubkey,
        group_member_address: Pubkey,
    ) -> Result<()> {
        let ix = spl_token_2022::extension::group_member_pointer::instruction::initialize(
            &token_program_id,
            &mint_account.key(),
            Some(authority),
            Some(group_member_address)
        )?;
    
        let account_infos: Vec<AccountInfo> = vec![
            mint_account.to_account_info(),
        ];
    
        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;
    
        Ok(())
    }

}
