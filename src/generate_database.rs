use crate::{
    database::{self, Candle, PgState, StateReader, StateWriter, Transaction},
    errors::MainProcessError,
    market,
    tools::{DatabaseGenerationConfiguration, YamlFile},
};
use market::Player;
use std::env;

pub async fn generate_database() -> Result<(), MainProcessError> {
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
