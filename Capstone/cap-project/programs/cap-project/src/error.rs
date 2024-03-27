use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorMessages {
    #[msg("Error1")]
    Error1,

    #[msg("Error2")]
    Error2,

    #[msg("Worker is none")]
    Error3,

    #[msg("Vault has not expired")]
    VaultNotExpired,
}
