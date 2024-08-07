use crate::database::StateReader;
use crate::database::StateWriter;
use crate::database::Transaction;
use crate::{
    database::{self, postgres_connection::PgState, Candle},
    errors::MainProcessError,
    trading_objects::{self, Market},
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use trading_objects::Player;

pub async fn generate_database(mut all_player: Vec<Player>) -> Result<(), MainProcessError> {
    let mut rng = thread_rng();
    all_player.shuffle(&mut rng);
    let mut market = Market::new(1000.);
    let mut all_tick = Vec::new();
    let binding = env::var("DATABASE_URL")?;
    let database_connector = PgState::try_new(&binding).await?;
    let mut candle_id: i32 = 0;
    loop {
        let mut tx: database::StateTransaction = database_connector.init_writer().await?;
        let playing_player = &mut all_player[0];
        let (buy_ids_to_remove, sell_ids_to_remove) = playing_player.removing_pending_orders();

        market
            .queue_pending_buy_order
            .retain(|s| !buy_ids_to_remove.contains(&s.id));

        market
            .queue_pending_sell_order
            .retain(|s| !sell_ids_to_remove.contains(&s.id));

        let mut all_sell_order_to_add_to_market = Vec::new();
        if playing_player.number_of_shares > 0 {
            let sell_id_to_add = playing_player.sell_shares(&market);
            if let Some(id) = sell_id_to_add {
                for item in playing_player.pending_sell_orders.iter() {
                    if id == item.id {
                        market.queue_pending_sell_order.push(item.clone());
                        all_sell_order_to_add_to_market.push(item.clone());
                    }
                }
            }
        }
        let mut all_buy_order_to_add_to_market = Vec::new();
        let buy_id_to_add = playing_player.buy_shares(&market)?;
        if let Some(id) = buy_id_to_add {
            for item in playing_player.pending_buy_orders.iter() {
                if id == item.id {
                    market.queue_pending_buy_order.push(item.clone());
                    all_buy_order_to_add_to_market.push(item.clone());
                }
            }
        }
        for sell_order in all_sell_order_to_add_to_market {
            market.find_buyer(&sell_order, &mut all_player);
            all_tick.push(market.market_price);
            if all_tick.len() == 100 {
                candle_id += 1;
                let new_candle = Candle::new_candle_from_tick(candle_id, &all_tick);
                tx.write_candle(new_candle).await?;
                all_tick.clear();
            }
        }
        for buy_order in all_buy_order_to_add_to_market {
            market.find_seller(&buy_order, &mut all_player);
            all_tick.push(market.market_price);
            if all_tick.len() == 100 {
                candle_id += 1;
                let new_candle = Candle::new_candle_from_tick(candle_id, &all_tick);
                tx.write_candle(new_candle).await?;
                all_tick.clear();
            }
        }
        all_player.shuffle(&mut rng);
        tx.commit().await?;
        println!("number of candle created: {}", candle_id);
    }
    Ok(())
}
