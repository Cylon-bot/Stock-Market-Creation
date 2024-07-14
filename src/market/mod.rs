use thiserror::Error;

mod player;

#[derive(Debug, Error)]
pub enum PlayerError {}

#[derive(Debug)]
pub struct Player {
    money: f64,

    probability_of_taking_share: f64,

    probability_of_retrieving_share: f64,
    // percentage of share retrieved from a trade
    amount_share_retrieved: f64,
    // taken share on a trade
    min_share_taken: i64,
    max_share_taken: i64,

    on_going_trades: Vec<Trade>,
}

#[derive(Debug)]
pub struct Trade {
    share_buying_price: f64,
    share_numbers: f64,
}
