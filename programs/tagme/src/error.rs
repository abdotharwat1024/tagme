use anchor_lang::prelude::*;

#[error_code]
pub enum TagMeError {
    #[msg("Unauthorized signer.")]
    Unauthorized,
    #[msg("Product is revoked.")]
    ProductRevoked,
    #[msg("Invalid metadata.")]
    InvalidMetadata,
    #[msg("History account missing.")]
    MissingHistory,
    #[msg("Invalid Name.")]
    InvalidName,
    #[msg("Invalid url.")]
    InvalidUrl,
    #[msg("Wrong history account.")]
    InvalidHistory,
    #[msg("Invalid self transfering.")]
    InvalidAccount,
}
