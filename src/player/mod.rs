enum OrderType {
    Sell,
    Buy,
}

struct Player {
    money: f64,
    current_position_type: OrderType,
}
