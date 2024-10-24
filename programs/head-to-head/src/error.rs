use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Invalid entrance fee. Must be 1000 USDC")]
    InvalidEntranceFee,
    #[msg("Invalid USDC token account")]
    InvalidTokenAccount,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Failed to reallocate account")]
    ReallocError,
    #[msg("Invalid price value. Must be > 0")]
    InvalidPrice,
    #[msg("Only admin can perform this action")] // NEW ERROR FOR ADMIN CHECK
    AdminOnly,
    #[msg("Game not found")]
    GameNotFound,
    #[msg("Only the host can withdraw")]
    UnauthorizedWithdrawal,
    #[msg("Cannot withdraw after opponent has joined")]
    WithdrawalNotAllowed,
    #[msg("Game is already closed")]
    GameAlreadyClosed,
    #[msg("Game not started yet")]
    GameNotStarted,
    #[msg("Game already has an opponent")]
    GameAlreadyJoined,
    #[msg("You cannot join your own game")]
    CannotJoinOwnGame,
    #[msg("Price moved too much since game creation")]
    PriceMovedTooMuch,
    #[msg("Game is not finished yet - 5% threshold not reached")]
    GameNotFinished,
    #[msg("Only winner can claim rewards")]
    SignerNotWinner,
}
