mod database;
mod player;
mod tools;

use dotenv::dotenv;
use std::env::{self, VarError};
use thiserror::Error;

use database::{Candle, DatabaseError, PgError, PgState, StateReader, StateWriter, Transaction};
use player::Player;

#[derive(Debug, Error)]
enum MainProcessError {
    #[error(transparent)]
    EnvError(#[from] VarError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}

#[tokio::main]
async fn main() -> Result<(), MainProcessError> {
    dotenv().ok();
    let new_player = Player::new(50., 0.5, 0.5);
    let new_candle = Candle::new(0, 1., 1., 1., 1.);
    let binding = env::var("DATABASE_URL")?;
    let database_connector = PgState::try_new(&binding).await?;
    // let mut tx = database_connector.init_writer().await?;
    // tx.write_candle(new_candle);
    // tx.commit();
    let a = match database_connector.read_candle_by_id(0).await? {
        Some(a) => a,
        None => todo!(":("),
    };
    println!("candle: {:?}", a);
    Ok(())
}
