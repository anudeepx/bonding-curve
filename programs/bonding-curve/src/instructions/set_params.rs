use crate::{constants::GLOBAL_SEED, error::ErrorCode, state::Global};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global_state.bump,
        constraint = global_state.authority == admin.key() @ ErrorCode::NotAuthorized
    )]
    pub global_state: Box<Account<'info, Global>>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetParams<'info> {
    pub fn handler(
        &mut self,
        fee_recipient: Option<Pubkey>,
        initial_virtual_token_reserves: Option<u64>,
        initial_virtual_sol_reserves: Option<u64>,
        initial_real_token_reserves: Option<u64>,
        token_total_supply: Option<u64>,
        fee_basis_points: Option<u16>,
    ) -> Result<()> {
        require!(
            fee_basis_points.map_or(true, |fbp| fbp <= 10_000),
            ErrorCode::InvalidFeeBasisPoints
        );

        let global = &mut self.global_state;

        global.fee_recipient = fee_recipient.unwrap_or(global.fee_recipient);
        global.initial_virtual_token_reserves =
            initial_virtual_token_reserves.unwrap_or(global.initial_virtual_token_reserves);
        global.initial_virtual_sol_reserves =
            initial_virtual_sol_reserves.unwrap_or(global.initial_virtual_sol_reserves);
        global.initial_real_token_reserves =
            initial_real_token_reserves.unwrap_or(global.initial_real_token_reserves);
        global.token_total_supply = token_total_supply.unwrap_or(global.token_total_supply);
        global.fee_basis_points = fee_basis_points.unwrap_or(global.fee_basis_points);

        Ok(())
    }
}
