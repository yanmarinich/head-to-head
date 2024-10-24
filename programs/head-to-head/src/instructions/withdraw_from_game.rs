use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };
use crate::{
    constants::{ self, VAULT_SEED },
    state::{ Config, Games },
    utils::transfer_tokens,
    error::Error,
};

#[derive(Accounts)]
pub struct WithdrawFromGame<'info> {
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

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_from_game(ctx: Context<WithdrawFromGame>, game_index: u32) -> Result<()> {
    let WithdrawFromGame {
        player,
        games_account,
        player_token_account,
        vault_account,
        token_program,
        ..
    } = ctx.accounts;

    require!((game_index as usize) < games_account.games.len(), Error::GameNotFound);

    let game = &mut games_account.games[game_index as usize];

    require!(game.host == player.key(), Error::UnauthorizedWithdrawal);
    require!(game.is_closed == false, Error::GameAlreadyClosed);
    require!(game.opponent.is_none(), Error::WithdrawalNotAllowed);

    transfer_tokens(
        vault_account.to_account_info(),
        player_token_account.to_account_info(),
        vault_account.to_account_info(),
        game.amount,
        token_program.to_account_info(),
        Some(&[&[VAULT_SEED, &[ctx.bumps.vault_account]]])
    )?;

    game.close();

    Ok(())
}
