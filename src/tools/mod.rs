pub mod mathematical;
pub mod yaml_connection;
use serde::{Deserialize, Serialize};
pub use yaml_connection::YamlFile;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseGenerationConfiguration {
    pub name_database: String,
    pub lower_time_frame: String,
    pub probability_of_buying: f64,
    pub probability_of_selling: f64,
    pub probability_of_removing_pending_order: f64,
    pub number_of_player: u64,
    pub number_of_market_share: u64,
}
