use anchor_lang::prelude::*;
use crate::constants::{ self };
use crate::state::Games;

#[derive(Accounts)]
pub struct InitializeGames<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [constants::GAMES_SEED], bump, payer = signer, space = 8 + 4)]
    pub games: Box<Account<'info, Games>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_games(_ctx: Context<InitializeGames>) -> Result<()> {
    Ok(())
}
