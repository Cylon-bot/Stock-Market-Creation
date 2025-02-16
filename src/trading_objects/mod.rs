use thiserror::Error;
use uuid::Uuid;

mod market;
mod player;

#[derive(Debug, Error)]
pub enum MarketError {}

#[derive(Debug, Clone)]
pub struct PendingBuyOrder {
    pub id: Uuid,
    number_of_shares: u64,
    id_player: Uuid,
    wanted_price: (f64, f64),
    alive: bool,
}

#[derive(Debug, Clone)]
pub struct PendingSellOrder {
    pub id: Uuid,
    number_of_shares: u64,
    id_player: Uuid,
    wanted_price: f64,
    alive: bool,
}
#[derive(Debug)]
pub struct Player {
    money: f64,
    pub id: Uuid,

    probability_of_buying: f64,
    probability_of_selling: f64,
    probability_of_removing_pending_order: f64,

    pub number_of_shares: u64,
    pub pending_buy_orders: Vec<PendingBuyOrder>,
    pub pending_sell_orders: Vec<PendingSellOrder>,
}

pub trait Order {
    fn get_id(&self) -> Uuid;
}

pub struct Market {
    pub market_price: f64,
    pub queue_pending_buy_order: Vec<PendingBuyOrder>,
    pub queue_pending_sell_order: Vec<PendingSellOrder>,
}
