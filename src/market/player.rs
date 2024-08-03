use rand::{thread_rng, Rng};

use crate::tools::mathematical::is_within_interval;

use super::{MarketError, Order, PendingBuyOrder, PendingSellOrder, Player};

impl Player {
    pub fn new(
        initial_money: f64,
        probability_of_buying: f64,
        probability_of_selling: f64,
        probability_of_removing_pending_order: f64,
    ) -> Player {
        Player {
            money: initial_money,
            probability_of_buying,
            probability_of_selling,
            probability_of_removing_pending_order,
            number_of_shares: 0,
            pending_buy_orders: Vec::new(),
            pending_sell_orders: Vec::new(),
        }
    }

    fn modify_pending_orders<T: Order + Clone>(&self, orders: &Vec<T>) -> Vec<T> {
        let mut rng = thread_rng();
        let mut new_pending_orders: Vec<T> = Vec::new();
        for pending_order in orders {
            let probability_number = rng.gen_range(0.0..=1.);
            let is_order_needs_to_be_removed = is_within_interval(
                probability_number as f64,
                0.,
                self.probability_of_removing_pending_order,
            );
            if !is_order_needs_to_be_removed {
                new_pending_orders.push(pending_order.clone());
            }
        }
        new_pending_orders
    }
    pub fn removing_pending_orders(&mut self) {
        self.pending_buy_orders = self.modify_pending_orders(&self.pending_buy_orders);
        self.pending_sell_orders = self.modify_pending_orders(&self.pending_sell_orders);
    }

    pub fn selling_shares(&mut self) {
        let mut rng = thread_rng();
        let probability_number = rng.gen_range(0.0..=1.);
        let sell_share: bool =
            is_within_interval(probability_number as f64, 0., self.probability_of_selling);
        if sell_share {
            let number_of_share_to_sell = rng.gen_range(1..=self.number_of_shares);
        }
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
impl Order for PendingBuyOrder {}
impl Order for PendingSellOrder {}
impl PendingSellOrder {
    pub fn new(shares_numbers: i64, wanted_price: f64) -> Result<PendingSellOrder, MarketError> {
        Ok(PendingSellOrder {
            shares_numbers,
            wanted_price,
        })
    }
}
