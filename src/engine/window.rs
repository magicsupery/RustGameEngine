
use crate::engine;
use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize, CreationError, Event, WindowEvent};

pub struct GameWindow {
    pub event_loop :  EventsLoop,
    window : Window,
}


impl GameWindow {
    pub fn new(width: f64, height: f64, title: &str) -> Result<Self, CreationError> {
        let event_loop = EventsLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize{width, height})
            .build(&event_loop);


        output.map(|window|Self{
            event_loop,
            window,
        })
    }

    pub fn event_loop(&mut self) -> bool{
        let mut end_result = false;
        self.event_loop.poll_events(|event| match event
        {
            Event::WindowEvent {
                window_id, event
            } => {
                match event {
                    WindowEvent::CloseRequested => {end_result = true},
                    WindowEvent::KeyboardInput{device_id, input} =>{
                        engine::input::Input::get_instance().borrow_mut().on_keyboard_event(input);
                    },
                    _ => {}
                }
            },
            _ => {}
        });

        end_result
    }
    pub fn stop(&self){
    }
}

