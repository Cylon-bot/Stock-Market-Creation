use thiserror::Error;

mod player;

#[derive(Debug, Error)]
pub enum MarketError {}

#[derive(Debug, Clone)]
pub struct PendingBuyOrder {
    shares_numbers: i64,
    wanted_price: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct PendingSellOrder {
    shares_numbers: i64,
    wanted_price: f64,
}
#[derive(Debug)]
pub struct Player {
    pub money: f64,

    pub probability_of_buying: f64,
    pub probability_of_selling: f64,
    pub probability_of_removing_pending_order: f64,

    pub number_of_shares: i64,
    pub pending_buy_orders: Vec<PendingBuyOrder>,
    pub pending_sell_orders: Vec<PendingSellOrder>,
}

pub trait Order {}

pub struct Market {
    market_price: f64,
}
