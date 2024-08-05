use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::tools::mathematical::is_within_interval;

use super::{Market, MarketError, Order, PendingBuyOrder, PendingSellOrder, Player};

impl Player {
    pub fn new(
        initial_money: f64,
        probability_of_buying: f64,
        probability_of_selling: f64,
        probability_of_removing_pending_order: f64,
    ) -> Player {
        Player {
            money: initial_money,
            id: Uuid::new_v4(),
            probability_of_buying,
            probability_of_selling,
            probability_of_removing_pending_order,
            number_of_shares: 0,
            pending_buy_orders: Vec::new(),
            pending_sell_orders: Vec::new(),
        }
    }

    fn modify_pending_orders<T: Order + Clone>(&self, orders: &Vec<T>) -> Vec<Uuid> {
        let mut rng = thread_rng();
        let mut id_orders_to_remove = Vec::new();
        for pending_order in orders {
            let probability_number = rng.gen_range(0.0..=1.);
            let is_order_needs_to_be_removed = is_within_interval(
                probability_number as f64,
                0.,
                self.probability_of_removing_pending_order,
            );
            if is_order_needs_to_be_removed {
                id_orders_to_remove.push(pending_order.get_id());
            }
        }
        id_orders_to_remove
    }
    pub fn removing_pending_orders(&mut self) -> (Vec<Uuid>, Vec<Uuid>) {
        let buy_orders_ids_to_remove = self.modify_pending_orders(&self.pending_buy_orders);
        self.pending_buy_orders
            .retain(|s| !buy_orders_ids_to_remove.contains(&s.id));

        let sell_orders_ids_to_remove = self.modify_pending_orders(&self.pending_sell_orders);
        for sell_order in &self.pending_sell_orders {
            if sell_orders_ids_to_remove.contains(&sell_order.id) {
                self.number_of_shares += sell_order.number_of_shares;
            }
        }
        self.pending_sell_orders
            .retain(|s| !sell_orders_ids_to_remove.contains(&s.id));
        (buy_orders_ids_to_remove, sell_orders_ids_to_remove)
    }

    pub fn sell_shares(&mut self, market: &Market) -> Option<Uuid> {
        let mut rng = thread_rng();
        let probability_number = rng.gen_range(0.0..=1.);
        let sell_share: bool =
            is_within_interval(probability_number as f64, 0., self.probability_of_selling);
        if sell_share {
            let number_of_share_to_sell = rng.gen_range(1..=self.number_of_shares);
            let wanted_price =
                market.market_price + rng.gen_range(-0.01..=0.01) * market.market_price;
            let new_sell_order =
                PendingSellOrder::new(wanted_price, self.id, number_of_share_to_sell);
            let id_to_return = new_sell_order.id;
            self.number_of_shares -= number_of_share_to_sell;
            self.pending_sell_orders.push(new_sell_order);
            return Option::Some(id_to_return);
        }
        return Option::None;
    }
    pub fn buy_shares(&mut self, market: &Market) -> Result<Option<Uuid>, MarketError> {
        let mut rng = thread_rng();
        let probability_number = rng.gen_range(0.0..=1.);
        let buy_share: bool =
            is_within_interval(probability_number as f64, 0., self.probability_of_buying);
        if buy_share {
            let number_of_share_to_buy = rng.gen_range(1..=1000);
            let wanted_price = (
                market.market_price - rng.gen_range(-0.0..=0.01) * market.market_price,
                market.market_price + rng.gen_range(-0.0..=0.01) * market.market_price,
            );
            let new_buy_order =
                PendingBuyOrder::try_new(wanted_price, self.id, number_of_share_to_buy)?;
            let id_to_return = new_buy_order.id;
            self.pending_buy_orders.push(new_buy_order);
            return Ok(Option::Some(id_to_return));
        }
        return Ok(Option::None);
    }
}

impl PendingBuyOrder {
    pub fn try_new(
        wanted_price: (f64, f64),
        id_player: Uuid,
        number_of_shares: u64,
    ) -> Result<PendingBuyOrder, MarketError> {
        match wanted_price {
            x if x.1 < x.0 => panic!("wanted price needs to be an interval and the first number needs to be equal or inferior to the secode one"),
            _ => (),
        }
        Ok(PendingBuyOrder {
            wanted_price,
            id: Uuid::new_v4(),
            id_player,
            number_of_shares,
        })
    }
}
impl Order for PendingBuyOrder {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
impl Order for PendingSellOrder {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
impl PendingSellOrder {
    pub fn new(wanted_price: f64, id_player: Uuid, number_of_shares: u64) -> PendingSellOrder {
        PendingSellOrder {
            wanted_price,
            id: Uuid::new_v4(),
            id_player,
            number_of_shares,
        }
    }
}
