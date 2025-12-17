pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EHsYksFv4a86eptVJoV56SPXdP7WNhnMYqoAbpzMXt26");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.handler(&ctx.bumps)
    }

    pub fn set_operating_state(
        ctx: Context<SetOperatingState>,
        new_state: OperatingState,
    ) -> Result<()> {
        ctx.accounts.handler(new_state)
    }

    pub fn set_params(
        ctx: Context<SetParams>,
        fee_recipient: Option<Pubkey>,
        initial_virtual_token_reserves: Option<u64>,
        initial_virtual_sol_reserves: Option<u64>,
        initial_real_token_reserves: Option<u64>,
        token_total_supply: Option<u64>,
        fee_basis_points: Option<u16>,
    ) -> Result<()> {
        ctx.accounts.handler(
            fee_recipient,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
        )
    }
}
