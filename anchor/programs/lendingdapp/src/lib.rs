#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use instructions::*;

mod state;
mod errors;
mod instructions;
declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod lendingdapp {
    use super::*;
    pub fn init_bank(cxt: Context<InitBank>) -> Result<()> {
        init_bank::handle_init_bank(cxt)
    }
    pub fn init_user(cxt: Context<InitUser>) -> Result<()> {
        init_user::handle_init_user(cxt)
    }
}

