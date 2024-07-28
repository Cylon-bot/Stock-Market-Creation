use super::{MarketError, PendingBuyOrder, PendingSellOrder, Player};

impl Player {
    pub fn try_new(
        initial_money: f64,
        probability_of_buying: f64,
        probability_of_selling: f64,
        probability_of_removing_pending_order: f64,
    ) -> Result<Player, MarketError> {
        Ok(Player {
            money: initial_money,
            probability_of_buying,
            probability_of_selling,
            probability_of_removing_pending_order,
            number_of_shares: 0,
            pending_buy_orders: Vec::new(),
            pending_sell_orders: Vec::new(),
        })
    }
}

impl PendingBuyOrder {
    pub fn try_new(
        shares_numbers: i64,
        wanted_price: (f64, f64),
    ) -> Result<PendingBuyOrder, MarketError> {
        match wanted_price {
            x if x.1 < x.0 => panic!("wanted price needs to be an interval and the first number needs to be equal or inferior to the secode one"),
            _ => (),
        }
        Ok(PendingBuyOrder {
            shares_numbers,
            wanted_price,
        })
    }
}

impl PendingSellOrder {
    pub fn new(shares_numbers: i64, wanted_price: f64) -> Result<PendingSellOrder, MarketError> {
        Ok(PendingSellOrder {
            shares_numbers,
            wanted_price,
        })
    }
}
