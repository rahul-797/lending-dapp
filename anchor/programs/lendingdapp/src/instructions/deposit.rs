use anchor_lang::prelude::*;
use crate::state::{Bank, User};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenAccount, Mint, TokenInterface};

pub fn handle_deposit(cxt: Context<Deposit>, amount: u64) -> Result<()> {
    let bank = &mut cxt.accounts.bank;
    let user = &mut cxt.accounts.user;

    if bank.total_deposits == 0 {
        bank.total_deposits = amount;
        bank.total_deposit_shares = amount;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user: Account<'info, User>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    #[account(
        mut,
        seeds = [b"bank", mint.key().as_ref()],
        bump,
    )]
    pub bank_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}