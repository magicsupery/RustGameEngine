
const DIMENTIONS: (u32, u32) = (800, 600);
const TITLE: &str = "3D Game Engine";

mod engine;

fn main() {
    let mut window = engine::window::create_window(DIMENTIONS.0, DIMENTIONS.1, TITLE);
    while window.running() {
       window.render();
    }

    window.stop();

}
