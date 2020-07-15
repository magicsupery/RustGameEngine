
mod engine;
use crate::engine::input;
fn main() {
    let mut vec = engine::vector2::Vector2::new(10.0, 20.0);
    println!("{}, length {} ", vec, vec.length());
    vec.normalize();
    println!("{}", vec);

    let vec2 = engine::vector2::Vector2::new(10.0, 20.0);
    println!("{}", vec + vec2 + 3.0 - 2.0 - vec / vec2 / 3.0);
    let mut game = engine::game::Game::new();
    game.start();
    game.run();
}
