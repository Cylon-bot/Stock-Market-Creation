use crate::tools::mathematical::is_within_interval;

use super::{Market, PendingBuyOrder, PendingSellOrder, Player};

impl Market {
    pub fn new(market_price: f64) -> Market {
        Market {
            market_price,
            queue_pending_buy_order: Vec::new(),
            queue_pending_sell_order: Vec::new(),
        }
    }
    pub fn find_seller(&mut self, buy_order: &PendingBuyOrder, all_player: &mut [Player]) {
        for sell_order in &mut self.queue_pending_sell_order {
            if is_within_interval(
                sell_order.wanted_price,
                buy_order.wanted_price.0,
                buy_order.wanted_price.1,
            ) {
                let (buy_order_alive, sell_order_alive) =
                    apply_transaction(buy_order, sell_order, all_player);

                self.market_price = sell_order.wanted_price;

                if !sell_order_alive {
                    sell_order.alive = false;
                }

                if !buy_order_alive {
                    break;
                }
            }
        }

        self.queue_pending_buy_order.retain(|order| order.alive);
        self.queue_pending_sell_order.retain(|order| order.alive);
    }
    pub fn find_buyer(&mut self, sell_order: &PendingSellOrder, all_player: &mut [Player]) {
        for buy_order in &mut self.queue_pending_buy_order {
            if is_within_interval(
                sell_order.wanted_price,
                buy_order.wanted_price.0,
                buy_order.wanted_price.1,
            ) {
                let (buy_order_alive, sell_order_alive) =
                    apply_transaction(buy_order, sell_order, all_player);

                self.market_price = sell_order.wanted_price;

                if !buy_order_alive {
                    buy_order.alive = false;
                }

                if !sell_order_alive {
                    break;
                }
            }
        }

        self.queue_pending_buy_order.retain(|order| order.alive);
        self.queue_pending_sell_order.retain(|order| order.alive);
    }
}

fn apply_transaction(
    buy_order: &PendingBuyOrder,
    sell_order: &PendingSellOrder,
    all_player: &mut [Player],
) -> (bool, bool) {
    match buy_order.number_of_shares.cmp(&sell_order.number_of_shares) {
        std::cmp::Ordering::Greater => {
            if let Some(selling_player) =
                all_player.iter_mut().find(|s| s.id == sell_order.id_player)
            {
                selling_player
                    .pending_sell_orders
                    .retain(|s| sell_order.id != s.id);
                selling_player.money +=
                    sell_order.number_of_shares as f64 * sell_order.wanted_price;
            }

            if let Some(buying_player) = all_player.iter_mut().find(|s| s.id == buy_order.id_player)
            {
                if let Some(player_buying_order) = buying_player
                    .pending_sell_orders
                    .iter_mut()
                    .find(|s| s.id == sell_order.id)
                {
                    player_buying_order.number_of_shares -= sell_order.number_of_shares;
                }
                buying_player.number_of_shares += sell_order.number_of_shares;
                buying_player.money -= sell_order.number_of_shares as f64 * sell_order.wanted_price;
            }
            (true, false)
        }

        std::cmp::Ordering::Less => {
            if let Some(selling_player) =
                all_player.iter_mut().find(|s| s.id == sell_order.id_player)
            {
                if let Some(player_selling_order) = selling_player
                    .pending_sell_orders
                    .iter_mut()
                    .find(|s| s.id == sell_order.id)
                {
                    player_selling_order.number_of_shares -= buy_order.number_of_shares;
                }
                selling_player.money += buy_order.number_of_shares as f64 * sell_order.wanted_price;
            }

            if let Some(buying_player) = all_player.iter_mut().find(|s| s.id == buy_order.id_player)
            {
                buying_player
                    .pending_buy_orders
                    .retain(|s| buy_order.id != s.id);
                buying_player.number_of_shares += buy_order.number_of_shares;
                buying_player.money -= buy_order.number_of_shares as f64 * sell_order.wanted_price;
            }
            (false, true)
        }

        std::cmp::Ordering::Equal => {
            if let Some(selling_player) =
                all_player.iter_mut().find(|s| s.id == sell_order.id_player)
            {
                selling_player
                    .pending_sell_orders
                    .retain(|s| sell_order.id != s.id);
                selling_player.money += buy_order.number_of_shares as f64 * sell_order.wanted_price;
            }
            if let Some(buying_player) = all_player.iter_mut().find(|s| s.id == buy_order.id_player)
            {
                buying_player
                    .pending_buy_orders
                    .retain(|s| buy_order.id != s.id);
                buying_player.number_of_shares += buy_order.number_of_shares;
                buying_player.money -= buy_order.number_of_shares as f64 * sell_order.wanted_price;
            }
            (false, false)
        }
    }
}
