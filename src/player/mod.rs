mod player_impl;

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
    // volume taken define a percentage of the money that the player have
    volume_taken: f32,
}
