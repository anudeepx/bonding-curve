use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Global {
    pub authority: Pubkey,
    pub operating_state: OperatingState,
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u16,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum OperatingState {
    Normal, // Everything works
    Halted, // Nothing allowed
}
