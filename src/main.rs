mod database;
mod errors;
mod trading_objects;
mod process;
mod tools;

use dotenv::dotenv;
use rand::{thread_rng, Rng};
use std::env;

use crate::errors::MainProcessError;
use trading_objects::Player;
use process::generate_database;
use tools::{DatabaseGenerationConfiguration, YamlFile};

#[tokio::main]
async fn main() -> Result<(), MainProcessError> {
    dotenv().ok();
    let mut rng = thread_rng();
    let configuration: DatabaseGenerationConfiguration =
        YamlFile::try_new(env::var("YAML_CONF")?)?.file_content;
    let mut all_player = Vec::new();
    for _ in 1..configuration.number_of_player {
        let initial_money = rng.gen_range(1000..=10000000);
        let probability_of_buying =
            configuration.probability_of_buying + rng.gen_range(-0.01..=0.01) as f64;
        let probability_of_selling =
            configuration.probability_of_selling + rng.gen_range(-0.01..=0.01);
        let probability_of_removing_pending_order =
            configuration.probability_of_removing_pending_order + rng.gen_range(-0.01..=0.01);
        let new_player = Player::new(
            initial_money as f64,
            probability_of_buying as f64,
            probability_of_selling as f64,
            probability_of_removing_pending_order as f64,
        );
        all_player.push(new_player);
    }

    generate_database(all_player).await?;
    Ok(())
}
