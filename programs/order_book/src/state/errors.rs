use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("Invalid base mint")]
    InvalidBaseMint,
    #[msg("Invalid quote mint")]
    InvalidQuoteMint,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
}