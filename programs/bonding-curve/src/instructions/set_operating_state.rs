use crate::{
    error::ErrorCode,
    {constants::GLOBAL_SEED, Global, OperatingState},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetOperatingState<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global_state.bump,
    )]
    pub global_state: Box<Account<'info, Global>>,
    #[account(mut)]
    pub admin: Signer<'info>,
}
impl<'info> SetOperatingState<'info> {
    pub fn handler(&mut self, new_state: OperatingState) -> Result<()> {
        let global = &mut self.global_state;

        require!(
            self.admin.key() == global.authority,
            ErrorCode::NotAuthorized
        );

        global.operating_state = new_state;

        Ok(())
    }
}
