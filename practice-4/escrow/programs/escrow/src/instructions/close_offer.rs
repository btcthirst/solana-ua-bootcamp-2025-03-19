use anchor_lang::prelude::*;

use anchor_spl::{token::CloseAccount, 
    token_interface::{
        close_account,
        transfer_checked,
        Mint, 
        TokenAccount, 
        TokenInterface, 
        TransferChecked}};
use crate::Offer;

#[derive(Accounts)]
pub struct CloseOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_and_close_vault_and_offer(context: Context<CloseOffer>) -> Result<()> {
    let offer = &mut context.accounts.offer;
    let vault = &mut context.accounts.vault;

    // Transfer the tokens from the vault back to the maker
    let seeds = &[offer.maker.as_ref(), offer.id.to_le_bytes().as_ref(), &[offer.bump]];
    let signer = &[&seeds[..]];

    let accounts = TransferChecked {
        from: vault.to_account_info(),
        mint: context.accounts.token_mint_a.to_account_info(),
        to: context.accounts.maker_token_account_a.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };

    let ctx = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        accounts,
        signer,
    );



    transfer_checked(
       ctx,
        vault.amount,
        context.accounts.token_mint_a.decimals,
    )?;
    // Close the vault account
    let close_accounts = CloseAccount {
        account: vault.to_account_info(),
        destination: context.accounts.maker.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };
    let ctx = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        close_accounts,
        signer,
    );
    close_account(ctx)?;
    

    Ok(())
}