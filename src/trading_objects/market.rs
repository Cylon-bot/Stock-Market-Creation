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
        let mut buy_order_alive: bool = true;
        let mut buy_order_total_remaining_share = buy_order.number_of_shares;
        for sell_order in &mut self.queue_pending_sell_order {
            if is_within_interval(
                sell_order.wanted_price,
                buy_order.wanted_price.0,
                buy_order.wanted_price.1,
            ) {
                let (buy_order_remaining_share, sell_order_remaining_share) = apply_transaction(
                    buy_order,
                    sell_order,
                    all_player,
                    buy_order_total_remaining_share,
                    sell_order.number_of_shares,
                );

                self.market_price = sell_order.wanted_price;
                sell_order.number_of_shares = sell_order_remaining_share;

                if sell_order_remaining_share == 0 {
                    sell_order.alive = false;
                }

                if buy_order_remaining_share == 0 {
                    buy_order_alive = false;
                    break;
                }
                buy_order_total_remaining_share = buy_order_remaining_share;
            }
        }
        if !buy_order_alive {
            self.queue_pending_buy_order
                .retain(|order| order.id != buy_order.id);
        } else if let Some(buy_order) = self
            .queue_pending_buy_order
            .iter_mut()
            .find(|s| s.id == buy_order.id)
        {
            buy_order.number_of_shares = buy_order_total_remaining_share;
        }
        self.queue_pending_sell_order.retain(|order| order.alive);
    }
    pub fn find_buyer(&mut self, sell_order: &PendingSellOrder, all_player: &mut [Player]) {
        let mut sell_order_alive = true;
        let mut sell_order_total_remaining_share = sell_order.number_of_shares;
        for buy_order in &mut self.queue_pending_buy_order {
            if is_within_interval(
                sell_order.wanted_price,
                buy_order.wanted_price.0,
                buy_order.wanted_price.1,
            ) {
                let (buy_order_remaining_share, sell_order_remaining_share) = apply_transaction(
                    buy_order,
                    sell_order,
                    all_player,
                    buy_order.number_of_shares,
                    sell_order_total_remaining_share,
                );

                self.market_price = sell_order.wanted_price;
                buy_order.number_of_shares = buy_order_remaining_share;

                if buy_order_remaining_share == 0 {
                    buy_order.alive = false;
                }
                if sell_order_remaining_share == 0 {
                    sell_order_alive = false;
                    break;
                }

                sell_order_total_remaining_share = sell_order_remaining_share;
            }
        }
        self.queue_pending_buy_order.retain(|order| order.alive);
        if !sell_order_alive {
            self.queue_pending_sell_order
                .retain(|order| order.id != sell_order.id);
        } else if let Some(sell_order) = self
            .queue_pending_sell_order
            .iter_mut()
            .find(|s| s.id == sell_order.id)
        {
            sell_order.number_of_shares = sell_order_total_remaining_share;
        }
    }
}

fn apply_transaction(
    buy_order: &PendingBuyOrder,
    sell_order: &PendingSellOrder,
    all_player: &mut [Player],
    remaining_buy_share: u64,
    remaining_sell_share: u64,
) -> (u64, u64) {
    match remaining_buy_share.cmp(&remaining_sell_share) {
        std::cmp::Ordering::Greater => {
            let selling_player = if let Some(selling_player) =
                all_player.iter_mut().find(|s| s.id == sell_order.id_player)
            {
                selling_player
            } else {
                panic!("Player not found");
            };

            selling_player
                .pending_sell_orders
                .retain(|s| sell_order.id != s.id);
            selling_player.money += remaining_sell_share as f64 * sell_order.wanted_price;
            let buying_player = if let Some(buying_player) =
                all_player.iter_mut().find(|s| s.id == buy_order.id_player)
            {
                buying_player
            } else {
                panic!("Player not found");
            };
            let buying_order_number_of_shares = {
                buying_player.number_of_shares += remaining_sell_share;
                buying_player.money -= remaining_sell_share as f64 * sell_order.wanted_price;
                if let Some(player_buying_order) = buying_player
                    .pending_buy_orders
                    .iter_mut()
                    .find(|s| s.id == buy_order.id)
                {
                    player_buying_order.number_of_shares -= remaining_sell_share;
                    player_buying_order.number_of_shares
                } else {
                    panic!("Order not found");
                }
            };
            (buying_order_number_of_shares, 0)
        }

        std::cmp::Ordering::Less => {
            let selling_player = if let Some(selling_player) =
                all_player.iter_mut().find(|s| s.id == sell_order.id_player)
            {
                selling_player
            } else {
                panic!("Player not found");
            };
            let selling_order_number_of_shares = {
                selling_player.money += remaining_buy_share as f64 * sell_order.wanted_price;
                if let Some(player_selling_order) = selling_player
                    .pending_sell_orders
                    .iter_mut()
                    .find(|s| s.id == sell_order.id)
                {
                    player_selling_order.number_of_shares -= remaining_buy_share;
                    player_selling_order.number_of_shares
                } else {
                    panic!("Order not found");
                }
            };
            let buying_player = if let Some(buying_player) =
                all_player.iter_mut().find(|s| s.id == buy_order.id_player)
            {
                buying_player
            } else {
                panic!("Player not found");
            };
            buying_player
                .pending_buy_orders
                .retain(|s| buy_order.id != s.id);
            buying_player.number_of_shares += remaining_buy_share;
            buying_player.money -= remaining_buy_share as f64 * sell_order.wanted_price;
            (0, selling_order_number_of_shares)
        }

        std::cmp::Ordering::Equal => {
            let selling_player = if let Some(selling_player) =
                all_player.iter_mut().find(|s| s.id == sell_order.id_player)
            {
                selling_player
            } else {
                panic!("Player not found");
            };
            selling_player
                .pending_sell_orders
                .retain(|s| sell_order.id != s.id);
            selling_player.money += remaining_buy_share as f64 * sell_order.wanted_price;
            let buying_player = if let Some(buying_player) =
                all_player.iter_mut().find(|s| s.id == buy_order.id_player)
            {
                buying_player
            } else {
                panic!("Player not found");
            };
            buying_player
                .pending_buy_orders
                .retain(|s| buy_order.id != s.id);
            buying_player.number_of_shares += remaining_buy_share;
            buying_player.money -= remaining_buy_share as f64 * sell_order.wanted_price;
            (0, 0)
        }
    }
}
