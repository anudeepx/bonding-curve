use anchor_lang::prelude::*;
use crate::{{Global, OperatingState, constants::GLOBAL_SEED}, error::ErrorCode};

#[derive(Accounts)]
pub struct SetOperatingState<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global_state.bump,
    )]
    pub global_state: Account<'info, Global>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
impl<'info> SetOperatingState<'info> {
    pub fn handler(&mut self, new_state: OperatingState) -> Result<()> {
        let global = &mut self.global_state;

        require!(
            self.authority.key() == global.authority,
            ErrorCode::NotAuthorized
        );

        global.operating_state = new_state;

        Ok(())
    }
}