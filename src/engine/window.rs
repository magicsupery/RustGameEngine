
use crate::engine;
use winit::window::{Window, WindowBuilder};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::error::OsError;
use winit::platform::desktop::EventLoopExtDesktop;

pub struct GameWindow {
    pub event_loop :  EventLoop<()>,
    pub window : Window,
}


impl GameWindow {
    pub fn new(width: f64, height: f64, title: &str) -> Result<Self, OsError> {
        let event_loop = EventLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(
                width,
                height,
            )))
            .build(&event_loop);


        output.map(|window|Self{
            event_loop,
            window,
        })
    }

    pub fn event_loop(&mut self) -> bool{
        let mut end_result = false;
        self.event_loop.run_return(|event, _, control_flow|
        {
            match event{
                Event::WindowEvent {
                    window_id, event
                } => {
                    match event {
                        WindowEvent::CloseRequested => {end_result = true},
                        WindowEvent::KeyboardInput{device_id, input, is_synthetic } =>{
                            engine::input::Input::get_instance().borrow_mut().on_keyboard_event(input);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            *control_flow = ControlFlow::Exit;
        });

        end_result
    }
    pub fn stop(&self){
    }
}

