use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::{
            instructions::{
                CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
                CreateMetadataAccountV3InstructionArgs,
            },
            types::DataV2,
        },
        Metadata,
    },
    token::{self, Mint, Token, TokenAccount},
};

use crate::{error::ErrorCode, Global, TokenCreated};
use crate::{
    BondingCurve, OperatingState, BONDING_CURVE_SEED, BONDING_CURVE_VAULT_SEED, GLOBAL_SEED,
    METADATA_SEED,
};

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        mint::decimals = 6,
        mint::authority = bonding_curve,
        mint::freeze_authority = bonding_curve,
        mint::token_program = token_program
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump = global_state.bump,
        constraint = global_state.operating_state == OperatingState::Normal @ ErrorCode::InvalidOperatingState
    )]
    pub global_state: Box<Account<'info, Global>>,

    #[account(
        init,
        payer = user,
        space = 8 + BondingCurve::INIT_SPACE,
        seeds = [BONDING_CURVE_SEED, mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    // SOL vault
    #[account(
        mut,
        seeds = [BONDING_CURVE_VAULT_SEED, mint.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve
    )]
    pub bonding_curve_ata: Account<'info, TokenAccount>,

    /// CHECK: Metaplex metadata PDA
    #[account(
        mut,
        seeds = [
            METADATA_SEED,
            mpl_metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = mpl_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub mpl_metadata_program: Program<'info, Metadata>,
}

impl<'info> Create<'info> {
    pub fn handler(
        &mut self,
        name: String,
        symbol: String,
        uri: String,
        bump: u8,
        vault_bump: u8,
    ) -> Result<()> {
        let seeds = &[
            BONDING_CURVE_SEED,
            &self.mint.to_account_info().key.as_ref(),
            &[bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let metadata = &self.metadata.to_account_info();
        let mint = &self.mint.to_account_info();
        let mint_authority = &self.bonding_curve.to_account_info();
        let payer = &self.user.to_account_info();
        let update_authority = &self.bonding_curve.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let token_metadata_program = &self.mpl_metadata_program.to_account_info();
        let name1 = name.clone();
        let symbol1 = symbol.clone();
        let uri1 = uri.clone();

        CreateMetadataAccountV3Cpi::new(
            token_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority,
                update_authority: (update_authority, true),
                payer,
                system_program,
                rent: Some(&self.rent.to_account_info()),
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        )
        .invoke_signed(signer_seeds)?;

        token::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::MintTo {
                    authority: self.bonding_curve.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.bonding_curve_ata.to_account_info(),
                },
                signer_seeds,
            ),
            self.global_state.token_total_supply,
        )?;

        token::set_authority(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::SetAuthority {
                    account_or_mint: self.mint.to_account_info(),
                    current_authority: self.bonding_curve.to_account_info(),
                },
                signer_seeds,
            ),
            token::spl_token::instruction::AuthorityType::MintTokens,
            None,
        )?;

        self.bonding_curve.set_inner(BondingCurve {
            mint: self.mint.key(),
            virtual_token_reserve: self.global_state.initial_virtual_token_reserves,
            virtual_sol_reserve: self.global_state.initial_virtual_sol_reserves,
            real_token_reserve: self.global_state.initial_real_token_reserves,
            real_sol_reserve: 0,
            token_total_supply: self.global_state.token_total_supply,
            complete: false,
            initializer: self.user.key(),
            bump,
            vault_bump,
        });

        emit!(TokenCreated {
            mint: self.mint.key(),
            bonding_curve: self.bonding_curve.key(),
            user: self.user.key(),
            name: name1,
            symbol: symbol1,
            uri: uri1,
        });
        Ok(())
    }
}
