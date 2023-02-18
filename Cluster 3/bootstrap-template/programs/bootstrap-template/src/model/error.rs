use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Authority must be owned by XXX")]
    InvalidAuthority,
    #[msg("Account already initialized")]
    AlreadyInitialized
}