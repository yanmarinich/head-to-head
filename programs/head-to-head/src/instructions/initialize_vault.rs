use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };

use crate::{ constants, state::Config };

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [constants::CONFIG_SEED], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [constants::VAULT_SEED],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    // #[account(constraint = config.mint == mint.key())]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_vault(_ctx: Context<InitializeVault>) -> Result<()> {
    Ok(())
}
