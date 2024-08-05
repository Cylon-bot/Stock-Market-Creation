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
    pub fn find_seller(&mut self, buy_order: PendingBuyOrder, mut all_player: Vec<Player>) {
        for sell_order in &self.queue_pending_sell_order {
            if is_within_interval(
                sell_order.wanted_price,
                buy_order.wanted_price.0,
                buy_order.wanted_price.1,
            ) {
                match buy_order.number_of_shares.cmp(&sell_order.number_of_shares) {
                    std::cmp::Ordering::Greater => {
                        let _a = 3;
                    }
                    std::cmp::Ordering::Less => {
                        let _a = 3;
                    }

                    std::cmp::Ordering::Equal => {
                        if let Some(selling_player) =
                            all_player.iter_mut().find(|s| s.id == sell_order.id_player)
                        {
                            selling_player
                                .pending_sell_orders
                                .retain(|s| sell_order.id != s.id);
                            selling_player.money +=
                                buy_order.number_of_shares as f64 * sell_order.wanted_price;
                        }
                        if let Some(buying_player) =
                            all_player.iter_mut().find(|s| s.id == buy_order.id_player)
                        {
                            buying_player
                                .pending_buy_orders
                                .retain(|s| buy_order.id != s.id);
                            buying_player.number_of_shares += buy_order.number_of_shares;
                            buying_player.money -=
                                buy_order.number_of_shares as f64 * sell_order.wanted_price;
                        }
                    }
                }
            }
        }
    }
    pub fn find_buyer(&mut self, sell_order: PendingSellOrder) {
        for buy_order in &self.queue_pending_buy_order {}
    }
}
