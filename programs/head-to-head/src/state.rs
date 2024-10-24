use anchor_lang::prelude::*;

use crate::utils::check_price_fluctuation;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub bet_size: u64,
    pub win_threshold_percent: u16,
    pub join_threshold_percent: u16,
    pub threshold_decimals: u8,
}

impl Config {
    pub fn len() -> usize {
        8 + 32 + 32 + 8 + 2 + 2 + 1
    }
}

// TODO: make a separate program that stores realtime prices from pyth or chainlink to create price history
#[account]
pub struct Prices {
    pub prices: Vec<u64>,
    pub decimals: u8,
}

#[account]
pub struct Games {
    pub games: Vec<Game>,
}

#[account]
pub struct Game {
    pub host: Pubkey,
    pub opponent: Option<Pubkey>,
    pub host_prediction: bool,
    pub amount: u64,
    pub price_index: u32,
    pub result: Option<bool>,
    pub is_closed: bool,
}

impl Game {
    pub fn new(host: Pubkey, host_prediction: bool, amount: u64, price_index: u32) -> Self {
        Self {
            host,
            opponent: None,
            host_prediction,
            amount,
            price_index,
            result: None,
            is_closed: false,
        }
    }

    pub fn set_result(&mut self, result: bool) {
        self.result = Some(result);
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn join(&mut self, opponent: Pubkey) {
        self.opponent = Some(opponent);
    }

    pub fn check_price_fluctuation(
        &mut self,
        prices: &Vec<u64>,
        max_percentage: u16,
        price_decimals: u8,
        percentage_decimals: u8
    ) -> Option<bool> {
        check_price_fluctuation(
            prices,
            self.price_index as usize,
            max_percentage,
            price_decimals,
            percentage_decimals
        )
    }
}
