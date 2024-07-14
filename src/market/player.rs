use super::{Player, PlayerError};

impl Player {
    pub fn try_new(
        initial_money: f64,
        probability_of_taking_share: f64,
        probability_of_retrieving_share: f64,
        amount_share_retrieved: f64,
        min_share_taken: i64,
        max_share_taken: i64,
    ) -> Result<Player, PlayerError> {
        match min_share_taken {
            x if x < 1 => panic!("min_share_taken can't be below 1"),
            _ => (),
        }
        match max_share_taken {
            x if x < min_share_taken => {
                panic!("max share taken needs to be at least equal to min_share_taken")
            }
            _ => (),
        }
        Ok(Player {
            money: initial_money,
            probability_of_taking_share,
            probability_of_retrieving_share,
            amount_share_retrieved,
            min_share_taken,
            max_share_taken,
            on_going_trades: Vec::new(),
        })
    }
}
