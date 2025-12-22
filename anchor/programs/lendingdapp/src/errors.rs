use anchor_lang::prelude::*;

#[error_code]
pub enum LendingDappError {
    #[msg("Insufficient funds in bank vault.")]
    InsufficientFunds,
    #[msg("User account is not initialized.")]
    UserNotInitialized,
    #[msg("Bank account is not initialized.")]
    BankNotInitialized,
    #[msg("Invalid mint provided.")]
    InvalidMint,
}