use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::Bank;

pub fn handle_init_bank(cxt: Context<InitBank>) -> Result<()> {
    let bank = &mut cxt.accounts.bank;
    bank.mint_address = cxt.accounts.mint_address.key();
    bank.authority = cxt.accounts.signer.key();
    bank.total_deposits = 0;
    bank.total_deposit_shares = 0;
    bank.total_borrowed = 0;
    bank.total_borrowed_shares = 0;
    bank.liquidation_threshold = 8000; // 80%
    bank.liquidation_bonus = 10500; // 5% bonus
    bank.liquidation_close_factor = 5000; // 50%
    bank.max_ltv = 7500; // 75%
    bank.last_updated = Clock::get()?.unix_timestamp;
    bank.interest_rate = 300; // 3% APR
    Ok(())
}

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint_address: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = 8 + Bank::INIT_SPACE,
        seeds = [b"bank", mint_address.key().as_ref()],
        bump,
    )]
    pub bank: Account<'info, Bank>,
    #[account(
        init,
        payer = signer,
        token::mint = mint_address,
        token::authority = bank_vault,
        seeds = [b"bank_vault", mint_address.key().as_ref()],
        bump,
    )]
    pub bank_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}