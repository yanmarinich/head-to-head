use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::Token };
use crate::utils::resize_account;
use crate::{ constants::*, error::Error, state::* };

#[derive(Accounts)]
pub struct AddPrice<'info> {
    #[account(
        mut,
        address = config.admin @ Error::AdminOnly,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [PRICES_SEED],
        bump,
    )]
    pub prices: Account<'info, Prices>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn add_price(ctx: Context<AddPrice>, price: u64) -> Result<()> {
    let AddPrice { prices, .. } = ctx.accounts;

    require!(price > 0, Error::InvalidPrice);

    resize_account(
        prices,
        &ctx.accounts.admin,
        &ctx.accounts.system_program,
        std::mem::size_of::<u64>()
    )?;
    prices.prices.push(price);

    Ok(())
}
