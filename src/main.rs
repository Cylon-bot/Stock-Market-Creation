mod database;
mod errors;
mod process;
mod tools;
mod trading_objects;

use dotenv::dotenv;
use rand::{thread_rng, Rng};
use std::env;

use crate::errors::MainProcessError;
use process::generate_database;
use tools::{DatabaseGenerationConfiguration, YamlFile};
use trading_objects::Player;

#[tokio::main]
async fn main() -> Result<(), MainProcessError> {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
    let mut rng = thread_rng();
    let configuration: DatabaseGenerationConfiguration =
        YamlFile::try_new(env::var("YAML_CONF")?)?.file_content;
    let mut all_player = Vec::new();
    let mut remaining_shares = configuration.number_of_market_share;
    let max_shares = configuration.number_of_market_share;
    for player_key in 1..=configuration.number_of_player {
        let initial_money = rng.gen_range(1000..=10_000_000);
        let probability_of_buying =
            configuration.probability_of_buying + rng.gen_range(-0.01..=0.01) as f64;
        let probability_of_selling =
            configuration.probability_of_selling + rng.gen_range(-0.01..=0.01);
        let probability_of_removing_pending_order =
            configuration.probability_of_removing_pending_order + rng.gen_range(-0.01..=0.01);
        let mut player_number_of_shares;
        if remaining_shares >= (max_shares as f64 * 0.002) as u64 {
            player_number_of_shares = rng.gen_range(0..=(max_shares as f64 * 0.002) as u64) as u64;
        } else {
            player_number_of_shares = rng.gen_range(0..=remaining_shares);
        }
        if player_key == 1000 {
            player_number_of_shares = remaining_shares;
        }
        let new_player = Player::new(
            initial_money as f64,
            probability_of_buying as f64,
            probability_of_selling as f64,
            probability_of_removing_pending_order as f64,
            player_number_of_shares,
        );
        all_player.push(new_player);
        remaining_shares -= player_number_of_shares;
    }
    generate_database(all_player).await?;
    Ok(())
}
