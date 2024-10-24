use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token };
use crate::constants::{ self };
use crate::state::Config;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct InitializeConfigArgs {
    pub bet_size: u64,
    pub win_threshold_percent: u16,
    pub join_threshold_percent: u16,
    pub threshold_decimals: u8,
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [constants::CONFIG_SEED], bump, payer = signer, space = Config::len())]
    pub config: Box<Account<'info, Config>>,

    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_config(ctx: Context<InitializeConfig>, args: InitializeConfigArgs) -> Result<()> {
    let InitializeConfig { config, signer, mint, .. } = ctx.accounts;

    config.win_threshold_percent = args.win_threshold_percent;
    config.join_threshold_percent = args.join_threshold_percent;
    config.threshold_decimals = args.threshold_decimals;
    config.bet_size = args.bet_size;
    config.admin = signer.key();
    config.mint = mint.key();
    Ok(())
}
