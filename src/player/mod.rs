use rand::Rng;

#[derive(Debug)]
enum OrderType {
    Sell,
    Buy,
}
#[derive(Debug)]
pub struct Player {
    money: f64,
    current_position_type: OrderType,
    probability_of_changing_position: f32,
    probability_of_taking_trade: f32,
}

impl Player {
    pub fn new(
        initial_money: f64,
        probability_of_changing_position: f32,
        probability_of_taking_trade: f32,
    ) -> Player {
        let mut rng = rand::thread_rng();
        let random: u8 = rng.gen_range(0..2);
        let current_position_type = match random {
            0 => OrderType::Buy,
            1 => OrderType::Sell,
            _ => panic!("forbiden random value, this random value needs to be between 0 and 1"),
        };
        Player {
            money: initial_money,
            current_position_type,
            probability_of_changing_position,
            probability_of_taking_trade,
        }
    }
}
