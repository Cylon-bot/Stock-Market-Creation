use crate::{
    database::{self, Candle, PgState, StateReader, StateWriter, Transaction},
    errors::MainProcessError,
    player,
};
use player::Player;
use std::env;

pub async fn generate_database() -> Result<(), MainProcessError> {
    let binding = env::var("DATABASE_URL")?;
    let new_player = Player::new(50., 0.5, 0.5);
    let new_candle = Candle::new(2, 1., 1., 1., 1.);
    let database_connector = PgState::try_new(&binding).await?;
    let mut tx: database::StateTransaction = database_connector.init_writer().await?;
    tx.write_candle(new_candle).await?;
    tx.commit().await?;
    Ok(())
}
