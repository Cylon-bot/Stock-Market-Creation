pub mod yaml_connection;
pub mod mathematical;
use serde::{Deserialize, Serialize};
pub use yaml_connection::YamlFile;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseGenerationConfiguration {
    name_database: String,
}
