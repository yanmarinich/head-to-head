use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };

use crate::{
    constants::{ self, VAULT_SEED },
    error::Error,
    state::{ Config, Games, Prices },
    utils::transfer_tokens,
};

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(seeds = [constants::CONFIG_SEED], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        constraint = player_token_account.mint == config.mint,
        constraint = player_token_account.owner == player.key()
    )]
    pub player_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [constants::VAULT_SEED],
        bump,
        token::mint = config.mint,
    )]
    pub vault_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds = [constants::GAMES_SEED], bump)]
    pub games_account: Box<Account<'info, Games>>,

    #[account(seeds = [constants::PRICES_SEED], bump)]
    pub prices_account: Box<Account<'info, Prices>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn claim_winnings(ctx: Context<ClaimWinnings>, game_index: u32) -> Result<()> {
    let ClaimWinnings {
        player,
        games_account,
        prices_account,
        player_token_account,
        vault_account,
        config,
        token_program,
        ..
    } = ctx.accounts;

    require!((game_index as usize) < games_account.games.len(), Error::GameNotFound);

    let game = &mut games_account.games[game_index as usize];
    require!(!game.is_closed, Error::GameAlreadyClosed);
    require!(game.opponent.is_some(), Error::GameNotStarted);

    let result = game.check_price_fluctuation(
        &prices_account.prices,
        config.win_threshold_percent,
        prices_account.decimals,
        config.threshold_decimals
    );

    require!(result.is_some(), Error::GameNotFinished);
    let is_host_winner = game.host_prediction == result.unwrap();
    let winner_pubkey = if is_host_winner { game.host } else { game.opponent.unwrap() };
    require!(player.key() == winner_pubkey, Error::SignerNotWinner);

    transfer_tokens(
        vault_account.to_account_info(),
        player_token_account.to_account_info(),
        vault_account.to_account_info(),
        game.amount * 2,
        token_program.to_account_info(),
        Some(&[&[VAULT_SEED, &[ctx.bumps.vault_account]]])
    )?;

    game.set_result(result.unwrap());
    game.close();

    Ok(())
}
