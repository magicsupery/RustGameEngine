use crate::engine::{window, time, input, render};
use gfx_hal::Instance;
use std::mem::ManuallyDrop;

const DIMENSIONS: (f64, f64) = (800.0, 600.0);
const TITLE: &str = "3D Game Engine";

const FRAME_CAP : f64 = 9999.0;
const FRAME_TIME_LIMIT : f64 = 1.0 / FRAME_CAP;

pub struct Game {
    window: window::GameWindow,
    render: render::Render,
    start: bool,
    running: bool,
}

impl Game
{
    pub fn new() -> Game{
        let game_window = window::GameWindow::new(DIMENSIONS.0, DIMENSIONS.1, TITLE).
            expect("expect GameWindow");

        let render = render::Render::new(&game_window.window);

        Game {
            window: game_window,
            render,
            start: false,
            running: false
        }
    }

    pub fn start(&mut self){
        match self.start {
            true => {},
            false => {
                self.start = true;
                self.running = true;
            }
        }

    }

    pub fn run(&mut self) -> Result<(), String>{
        match self.start {
            false => Err(String::from("can not run with no start")),
            true => {
                let mut last_time = time::now();
                let mut unprocessed_time: f64 = 0.0;

                let mut frames = 0;
                let mut frameCounter: u128 = 0;

                loop{

                    let ended_result = self.window.event_loop();
                    if ended_result {
                        break;
                    }
                    let start_time = time::now();
                    let passed_time = start_time - last_time;
                    last_time = start_time;
                    unprocessed_time += passed_time as f64 / time::SECOND as f64;
                    frameCounter += passed_time;
                    // 非常困惑这里的含义

                    while unprocessed_time > FRAME_TIME_LIMIT{
                        unprocessed_time -= FRAME_TIME_LIMIT;
                        unsafe{
                            time::set_delta(FRAME_TIME_LIMIT);
                        }

                        //game update
                        self.update();

                        if frameCounter > time::SECOND {
                            println!("frame is {}", frames);
                            frameCounter = 0;
                            frames = 0;
                        }
                    }

                    self.render();
                    frames += 1;
                }

                self.stop();
                Ok(())
            }
        }
    }

    fn render(&mut self){
        self.render.render();
    }

    fn stop(&mut self){
        self.running = false;
        println!("game stopped")
    }

    fn update(&self){

    }

}