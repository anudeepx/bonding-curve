use anchor_lang::prelude::*;

use crate::{
    constants::{BONDING_CURVE_SUPPLY, GLOBAL_SEED, LAMPORTS_PER_SOL, P, R, SCALE, TOTAL_SUPPLY},
    error::ErrorCode,
    program::BondingCurve,
    {Global, OperatingState},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Global::INIT_SPACE,
        seeds = [GLOBAL_SEED],
        bump,
    )]
    pub global_state: Account<'info, Global>,

    pub system_program: Program<'info, System>,

    #[account(
        constraint = this_program.programdata_address()? == Some(program_data.key())
    )]
    pub this_program: Program<'info, BondingCurve>,

    #[account(
        constraint = program_data.upgrade_authority_address == Some(user.key())
            @ ErrorCode::NotAuthorized
    )]
    pub program_data: Account<'info, ProgramData>,
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self, bump: &InitializeBumps) -> Result<()> {
        let global = &mut self.global_state;

        global.set_inner(Global {
            authority: self.user.key(),
            operating_state: OperatingState::Normal,
            fee_recipient: self.user.key(),
            initial_virtual_token_reserves: P * SCALE,
            initial_virtual_sol_reserves: R * LAMPORTS_PER_SOL,
            initial_real_token_reserves: BONDING_CURVE_SUPPLY,
            token_total_supply: TOTAL_SUPPLY,
            fee_basis_points: 100, // 1%
            bump: bump.global_state,
        });

        Ok(())
    }
}
