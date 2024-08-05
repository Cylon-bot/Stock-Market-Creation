use std::collections::HashSet;

use crate::{
    errors::MainProcessError,
    trading_objects::{self, Market},
};
use trading_objects::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn generate_database(mut all_player: Vec<Player>) -> Result<(), MainProcessError> {
    let mut rng = thread_rng();
    all_player.shuffle(&mut rng);
    let mut market = Market::new(1.);
    loop {
        let playing_player = &mut all_player[0];
        let (buy_ids_to_remove, sell_ids_to_remove) = playing_player.removing_pending_orders();

        market
            .queue_pending_buy_order
            .retain(|s| !buy_ids_to_remove.contains(&s.id));

        market
            .queue_pending_sell_order
            .retain(|s| !sell_ids_to_remove.contains(&s.id));

        if playing_player.number_of_shares > 0 {
            let sell_ids_to_add = playing_player.sell_shares(&market);
            if !sell_ids_to_add.is_empty() {
                let ids_set: HashSet<_> = sell_ids_to_add.iter().collect();
                for item in playing_player.pending_sell_orders.iter() {
                    if ids_set.contains(&item.id) {
                        market.queue_pending_sell_order.push(item.clone());
                    }
                }
            }
        }

        let buy_ids_to_add = playing_player.buy_shares(&market)?;
        if !buy_ids_to_add.is_empty() {
            let ids_set: HashSet<_> = buy_ids_to_add.iter().collect();
            for item in playing_player.pending_buy_orders.iter() {
                if ids_set.contains(&item.id) {
                    market.queue_pending_buy_order.push(item.clone());
                }
            }
        }

        all_player.shuffle(&mut rng);
    }
    // let binding = env::var("DATABASE_URL")?;
    // let b: DatabaseGenerationConfiguration =
    //     YamlFile::try_new(env::var("YAML_CONF")?)?.file_content;
    // println!("{:?}", b);
    // let new_player = Player::new(50., 0.5, 0.5);
    // let new_candle = Candle::new(2, 1., 1., 1., 1.);
    // let database_connector = PgState::try_new(&binding).await?;
    // let mut tx: database::StateTransaction = database_connector.init_writer().await?;
    // tx.write_candle(new_candle).await?;
    // tx.commit().await?;
    Ok(())
}
