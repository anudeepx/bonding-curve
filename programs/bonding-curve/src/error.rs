use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount provided")]
    InvalidAmount,
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    #[msg("Arithmetic underflow occurred")]
    Underflow,
    #[msg("Not authorized to perform this action")]
    NotAuthorized,
}
