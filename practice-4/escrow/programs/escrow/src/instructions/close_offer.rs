use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
        close_account, transfer_checked, TransferChecked, CloseAccount, Mint, TokenAccount, TokenInterface,
    };

use crate::{
    Offer,
    InterfaceAccount,
};

#[derive(Accounts)]
pub struct CloseOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub offer: Account<'info, Offer>,
    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account( mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn send_offered_tokens_to_maker(ctx: &Context<CloseOffer>) -> Result<()> {
    // Transfer the tokens back to the maker
    let return_tokens = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.maker_token_account_a.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        return_tokens,
    );

    transfer_checked(
        cpi_context,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )?;
    Ok(())
}

/// in work
/// Close the offer and transfer the tokens back to the maker
pub fn close_vault(ctx: Context<CloseOffer>) -> Result<()> {
    // Close the vault account
    let close_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        close_accounts,
    );
    close_account(cpi_context)?;    

    Ok(())
}
////// Close the vault account