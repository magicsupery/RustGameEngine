
mod engine;
use engine::input;
use log;
use fern;
use chrono;

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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}][{}:{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    //vec_test();
    //matrix_test();
    setup_logger();
    let mut game = engine::game::Game::new();
    game.start();
    game.run();
}
