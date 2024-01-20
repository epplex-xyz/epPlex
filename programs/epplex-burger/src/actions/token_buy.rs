use crate::*;


#[derive(Accounts)]
pub struct TokenBuy<'info> {

    #[account(mut)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    // #[account(
    // seeds = [
    // SEED_TOKEN_METADATA,
    // mint.key().as_ref()
    // ],
    // bump,
    // )]
    // pub token_metadata: Account<'info, TokenMetadata>,
    // #[account(
    // seeds = [SEED_PROGRAM_DELEGATE],
    // bump = program_delegate.bump,
    // )]
    // pub program_delegate: Account<'info, ProgramDelegate>,
    //
    // #[account(mut)]
    // pub buyer: Signer<'info>,
    //
    //
    // #[account(mut)]
    // /// CHECK
    // pub ata_buyer: UncheckedAccount<'info>,
    //
    // /// CHECK
    // #[account(mut)]
    // pub seller: UncheckedAccount<'info>,
    //
    // #[account(mut)]
    // /// CHECK
    // pub ata_seller: UncheckedAccount<'info>,
    //
    // pub rent: Sysvar<'info, Rent>,
    // pub system_program: Program<'info, System>,
    // pub token22_program: Program<'info, Token2022>,
    // pub associated_token: Program<'info, AssociatedToken>,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBuyParams {

}

impl TokenBuy<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenBuyParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: TokenBuyParams) -> Result<()> {
        //Transfer price
        // let ix = solana_program::system_instruction::transfer(
        //     &ctx.accounts.buyer.key(),
        //     &ctx.accounts.seller.key(),
        //     ctx.accounts.token_metadata.price,
        // );
        //
        // let account_infos: Vec<AccountInfo> = vec![
        //     ctx.accounts.buyer.to_account_info(),
        //     ctx.accounts.seller.to_account_info(),
        //     ctx.accounts.system_program.to_account_info(),
        // ];
        //
        // solana_program::program::invoke(
        //     &ix,
        //     &account_infos[..],
        // )?;
        //
        //
        // //Create
        // anchor_spl::associated_token::create(
        //     CpiContext::new(
        //         ctx.accounts.token22_program.to_account_info(),
        //         anchor_spl::associated_token::Create {
        //             payer: ctx.accounts.buyer.to_account_info(), // payer
        //             associated_token: ctx.accounts.ata_buyer.to_account_info(),
        //             authority: ctx.accounts.buyer.to_account_info(), // owner
        //             mint: ctx.accounts.mint.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //             token_program: ctx.accounts.token22_program.to_account_info(),
        //         }
        //     ),
        // )?;
        //
        //
        // let ix = spl_token_2022::instruction::transfer_checked(
        //     ctx.accounts.token22_program.key,
        //     &ctx.accounts.ata_seller.key(),
        //     &ctx.accounts.mint.key(),
        //     &ctx.accounts.ata_buyer.key(),
        //     &ctx.accounts.program_delegate.key(),
        //     &[
        //         &ctx.accounts.program_delegate.key()
        //     ],
        //     1,
        //     0)?;
        //
        // let account_infos: Vec<AccountInfo> = vec![
        //     ctx.accounts.ata_seller.to_account_info(),
        //     ctx.accounts.mint.to_account_info(),
        //     ctx.accounts.ata_buyer.to_account_info(),
        //     ctx.accounts.program_delegate.to_account_info()
        // ];
        //
        // let program_delegate_seeds = &[
        //     SEED_PROGRAM_DELEGATE,
        //     &[ctx.accounts.program_delegate.bump]
        // ];
        //
        // solana_program::program::invoke_signed(
        //     &ix,
        //     &account_infos[..],
        //     &[program_delegate_seeds]
        // )?;
        //
        // // disable listing
        // let metadata = &mut ctx.accounts.token_metadata;
        // metadata.for_sale = false;
        // metadata.price = 0;
        //

        Ok(())
    }

}
