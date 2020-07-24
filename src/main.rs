
mod engine;
use crate::engine::input;
fn vec_test(){
    let mut vec = engine::vector2::Vector2::new(10.0, 20.0);
    println!("{}, length {} ", vec, vec.length());
    vec.normalize();
    println!("{}", vec);
    let vec2 = engine::vector2::Vector2::new(10.0, 20.0);
    println!("{}", vec + vec2 + 3.0 - 2.0 - vec / vec2 / 3.0);
}

fn matrix_test(){
    let mut a = engine::matrix4::Matrix4::<f64>::new();
    a.m[0][1] = 10.0;
    a.m[0][3] = 1244.123123;
    println!("{}", a);

    println!("{}", a * a);
}

fn main() {
    //vec_test();
    //matrix_test();
    let mut game = engine::game::Game::new();
    game.start();
    game.run();
}
