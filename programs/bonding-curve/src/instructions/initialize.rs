use anchor_lang::prelude::*;
use crate::state::{CurveConfiguration};
use crate::constants::CONFIG_SEED;

#[derive(Accounts)]
pub struct InitializeCurveConfiguration<'info> {
    #[account(
        init,
        payer = admin,
        space = CurveConfiguration::LEN,
        seeds = [CONFIG_SEED.as_bytes()],
        bump
    )]
    pub dex_config_acc: Box<Account<'info, CurveConfiguration>>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info,System>
}

pub fn handler(ctx: Context<InitializeCurveConfiguration>, fee: u64) -> Result<()> {
    let config_account = &mut ctx.accounts.dex_config_acc;
    config_account.set_inner(CurveConfiguration {
        fees: fee,
    });
    Ok(())
}
