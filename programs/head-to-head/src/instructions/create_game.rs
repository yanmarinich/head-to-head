use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };
use crate::{
    constants,
    state::{ Config, Game, Games, Prices },
    utils::{ resize_account, transfer_tokens },
};

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(seeds = [constants::CONFIG_SEED], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(seeds = [constants::PRICES_SEED], bump)]
    pub prices_account: Box<Account<'info, Prices>>,

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
    pub system_program: Program<'info, System>,
}

pub fn create_game(ctx: Context<CreateGame>, prediction: bool) -> Result<()> {
    let CreateGame {
        config,
        prices_account: prices,
        games_account: games,
        player,
        player_token_account,
        system_program,
        vault_account,
        ..
    } = ctx.accounts;

    let new_game = Game::new(
        player.key(),
        prediction,
        config.bet_size,
        (prices.prices.len() as u32) - 1
    );

    resize_account(games, player, system_program, std::mem::size_of::<Game>())?;
    games.games.push(new_game);

    transfer_tokens(
        player_token_account.to_account_info(),
        vault_account.to_account_info(),
        player.to_account_info(),
        config.bet_size,
        ctx.accounts.token_program.to_account_info(),
        None
    )?;

    Ok(())
}
