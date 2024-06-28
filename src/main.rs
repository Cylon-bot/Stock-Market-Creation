mod database;
mod player;
mod tools;

use player::Player;

fn main() {
    let new_player = Player::new(50., 0.5, 0.5);
    println!("{:?}", new_player)
}
