use anchor_lang::prelude::*;
use crate::constants::{ self };
use crate::state::Prices;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct InitializePricesArgs {
    pub initial_price: u64,
    pub price_decimals: u8,
}

#[derive(Accounts)]
pub struct InitializePrices<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [constants::PRICES_SEED], bump, payer = signer, space = 8 + 4 + 8 + 1)]
    pub prices: Box<Account<'info, Prices>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_prices(ctx: Context<InitializePrices>, args: InitializePricesArgs) -> Result<()> {
    let InitializePrices { prices, .. } = ctx.accounts;
    let InitializePricesArgs { initial_price, price_decimals, .. } = args;

    prices.prices.push(initial_price);
    prices.decimals = price_decimals;

    Ok(())
}
