mod engine;

use crate::engine::game::Game;


fn main() {
    
    let mut game = Game::new().unwrap();
    game.run();
    println!("Hello, world!");
}
