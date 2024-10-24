use anchor_lang::prelude::*;

use instructions::*;

mod instructions;
mod constants;
mod error;
mod state;
mod utils;

#[cfg(test)]
mod tests;

declare_id!("J5D34or7JDTnQFvCX8PpiAbkoYrYAhza3Xczi28PZR8p");

#[program]
pub mod head_to_head {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        args: InitializeConfigArgs
    ) -> Result<()> {
        instructions::initialize_config(ctx, args)
    }

    pub fn initialize_games(ctx: Context<InitializeGames>) -> Result<()> {
        instructions::initialize_games(ctx)
    }

    pub fn initialize_prices(
        ctx: Context<InitializePrices>,
        args: InitializePricesArgs
    ) -> Result<()> {
        instructions::initialize_prices(ctx, args)
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault(ctx)
    }

    pub fn add_price(ctx: Context<AddPrice>, price: u64) -> Result<()> {
        instructions::add_price(ctx, price)
    }
    pub fn create_game(ctx: Context<CreateGame>, prediction: bool) -> Result<()> {
        instructions::create_game(ctx, prediction)
    }

    pub fn join_game(ctx: Context<JoinGame>, game_index: u32) -> Result<()> {
        instructions::join_game(ctx, game_index)
    }

    pub fn claim_winnings(ctx: Context<ClaimWinnings>, game_index: u32) -> Result<()> {
        instructions::claim_winnings(ctx, game_index)
    }

    pub fn withdraw_from_game(ctx: Context<WithdrawFromGame>, game_index: u32) -> Result<()> {
        instructions::withdraw_from_game(ctx, game_index)
    }
}
