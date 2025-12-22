use anchor_lang::prelude::*;

use crate::state::User;

pub fn handle_init_user(cxt: Context<InitUser>) -> Result<()> {
    let user = &mut cxt.accounts.user;
    user.owner = cxt.accounts.signer.key();
    user.deposited_sol = 0;
    user.deposited_sol_shares = 0;
    user.borrowed_sol = 0;
    user.borrowed_sol_shares = 0;
    user.deposited_usdc = 0;
    user.deposited_usdc_shares = 0;
    user.borrowed_usdc = 0;
    user.borrowed_usdc_shares = 0;
    user.usdc_address = Pubkey::default();
    user.health_factor = 0;
    user.last_updated = Clock::get()?.unix_timestamp;
    Ok(())
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [signer.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}