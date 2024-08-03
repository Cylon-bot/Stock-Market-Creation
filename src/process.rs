use crate::{
    database::{self, Candle, PgState, StateReader, StateWriter, Transaction},
    errors::MainProcessError,
    market,
    tools::{DatabaseGenerationConfiguration, YamlFile},
};
use market::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn generate_database(mut all_player: Vec<Player>) -> Result<(), MainProcessError> {
    let mut rng = thread_rng();
    all_player.shuffle(&mut rng);
    loop {
        let playing_player = &mut all_player[0];
        playing_player.removing_pending_orders();
        if playing_player.number_of_shares > 0 {
            playing_player.selling_shares();
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
