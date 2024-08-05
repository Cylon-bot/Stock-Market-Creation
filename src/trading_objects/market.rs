use super::Market;

impl Market {
    pub fn new(market_price: f64) -> Market {
        Market {
            market_price,
            queue_pending_buy_order: Vec::new(),
            queue_pending_sell_order: Vec::new(),
        }
    }
}
