mod database;
mod errors;
mod generate_database;
mod player;
mod tools;

use crate::errors::MainProcessError;
use dotenv::dotenv;
use generate_database::generate_database;

#[tokio::main]
async fn main() -> Result<(), MainProcessError> {
    dotenv().ok();
    generate_database().await?;
    Ok(())
}
