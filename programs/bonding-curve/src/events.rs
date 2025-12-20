use anchor_lang::prelude::*;

#[event]
pub struct TokenCreated {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
