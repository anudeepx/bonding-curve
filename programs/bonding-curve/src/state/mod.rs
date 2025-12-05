use anchor_lang::prelude::*;

#[account()]
pub struct CurveConfiguration {
    pub fees: u64
}

impl CurveConfiguration {
    pub const LEN: usize = 8 + 8; // discriminator + fees
}