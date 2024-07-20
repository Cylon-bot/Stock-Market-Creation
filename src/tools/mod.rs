pub mod mathematical;
pub mod yaml_connection;
use serde::{Deserialize, Serialize};
pub use yaml_connection::YamlFile;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseGenerationConfiguration {
    name_database: String,
    lower_time_frame: String,
    probability_of_buying: f64,
    probability_of_selling: f64,
    probability_of_doing_nothing: f64,
    number_of_player: u64,
}
