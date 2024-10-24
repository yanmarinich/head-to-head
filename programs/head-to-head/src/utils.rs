use anchor_lang::prelude::*;
use crate::error::Error;
use anchor_lang::system_program;
use anchor_spl::token::{ transfer, Transfer };
use rust_decimal::prelude::*;

pub fn resize_account<'info, T: AccountSerialize + AccountDeserialize + Owner + Clone>(
    account: &mut Account<'info, T>,
    payer: &Signer<'info>,
    system_program: &Program<'info, System>,
    additional_space: usize
) -> Result<()> {
    let account_info = account.to_account_info();
    let current_space = account_info.data_len();
    let new_space = current_space.checked_add(additional_space).ok_or(Error::ArithmeticOverflow)?;

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_space);
    let lamports_diff = new_minimum_balance.saturating_sub(account_info.lamports());

    if lamports_diff > 0 {
        system_program::transfer(
            CpiContext::new(system_program.to_account_info(), system_program::Transfer {
                from: payer.to_account_info(),
                to: account_info.clone(),
            }),
            lamports_diff
        )?;
    }

    account_info.realloc(new_space, false).map_err(|_| Error::ReallocError)?;

    Ok(())
}

pub fn transfer_tokens<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    token_program: AccountInfo<'info>,
    signer_seeds: Option<&[&[&[u8]]]>
) -> Result<()> {
    let cpi_accounts: Transfer = Transfer {
        from,
        to,
        authority,
    };

    let cpi_context: CpiContext<Transfer> = if let Some(seeds) = signer_seeds {
        CpiContext::new_with_signer(token_program, cpi_accounts, seeds)
    } else {
        CpiContext::new(token_program, cpi_accounts)
    };

    transfer(cpi_context, amount)
}

pub fn check_price_fluctuation(
    prices: &Vec<u64>,
    start_index: usize,
    max_percentage: u16,
    price_decimals: u8,
    percentage_decimals: u8
) -> Option<bool> {
    if start_index >= prices.len() {
        return None;
    }

    let d_start_price =
        Decimal::from(*prices.get(start_index)?) /
        Decimal::from((10u32).pow(price_decimals as u32));

    let d_max_percentage =
        Decimal::from(max_percentage) / Decimal::from((10u32).pow(percentage_decimals as u32));

    let up_threshold = d_start_price * (Decimal::ONE + d_max_percentage / Decimal::from(100));
    let down_threshold = d_start_price * (Decimal::ONE - d_max_percentage / Decimal::from(100));

    for &price in &prices[start_index + 1..] {
        let d_price = Decimal::from(price).checked_div(
            Decimal::from_u32((10_u32).pow(price_decimals as u32))?
        )?;

        if d_price >= up_threshold {
            return Some(true);
        } else if d_price <= down_threshold {
            return Some(false);
        }
    }

    None
}
