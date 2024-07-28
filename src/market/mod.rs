use thiserror::Error;

mod player;

#[derive(Debug, Error)]
pub enum MarketError {}
#[derive(Debug)]
pub struct PendingBuyOrder {
    shares_numbers: i64,
    wanted_price: (f64, f64),
}
#[derive(Debug)]
pub struct PendingSellOrder {
    shares_numbers: i64,
    wanted_price: f64,
}
#[derive(Debug)]
pub struct Player {
    money: f64,

    probability_of_buying: f64,
    probability_of_selling: f64,
    probability_of_removing_pending_order: f64,

    number_of_shares: i64,
    pending_buy_orders: Vec<PendingBuyOrder>,
    pending_sell_orders: Vec<PendingSellOrder>,
}
