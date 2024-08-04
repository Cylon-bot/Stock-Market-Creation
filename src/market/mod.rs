use thiserror::Error;
use uuid::Uuid;

mod market;
mod player;

#[derive(Debug, Error)]
pub enum MarketError {}

#[derive(Debug, Clone)]
pub struct PendingBuyOrder {
    pub id: Uuid,
    wanted_price: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct PendingSellOrder {
    pub id: Uuid,
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

pub trait Order {
    fn get_id(&self) -> Uuid;
}

pub struct Market {
    market_price: f64,
    queue_pending_buy_order: Vec<PendingBuyOrder>,
    queue_pending_sell_order: Vec<PendingSellOrder>,
}
