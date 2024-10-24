use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };

use crate::{ constants, error::Error, state::{ Config, Games, Prices }, utils::transfer_tokens };

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(seeds = [constants::CONFIG_SEED], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(seeds = [constants::PRICES_SEED], bump)]
    pub prices: Box<Account<'info, Prices>>,

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
        token::authority = vault_account
    )]
    pub vault_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds = [constants::GAMES_SEED], bump)]
    pub games_account: Box<Account<'info, Games>>,

    pub token_program: Program<'info, Token>,
}

pub fn join_game(ctx: Context<JoinGame>, game_index: u32) -> Result<()> {
    let JoinGame {
        player,
        games_account,
        player_token_account,
        vault_account,
        config,
        token_program,
        prices,
        ..
    } = ctx.accounts;

    require!((game_index as usize) < games_account.games.len(), Error::GameNotFound);

    let game = &mut games_account.games[game_index as usize];

    require!(!game.is_closed, Error::GameAlreadyClosed);
    require!(game.host != player.key(), Error::CannotJoinOwnGame);
    require!(game.opponent.is_none(), Error::GameAlreadyJoined);

    require!(
        game
            .check_price_fluctuation(
                &prices.prices,
                config.join_threshold_percent,
                prices.decimals,
                config.threshold_decimals
            )
            .is_none(),
        Error::PriceMovedTooMuch
    );

    transfer_tokens(
        player_token_account.to_account_info(),
        vault_account.to_account_info(),
        player.to_account_info(),
        config.bet_size,
        token_program.to_account_info(),
        None
    )?;

    game.join(player.key());

    Ok(())
}
