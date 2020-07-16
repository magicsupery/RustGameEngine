
use crate::engine;
use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize, CreationError, Event, WindowEvent};

pub struct GameWindow {
    event_loop :  EventsLoop,
    window : Window,
    running : bool,
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
            running: true,
        })
    }

    pub fn render(&mut self){
        let running = &mut self.running;
        self.event_loop.poll_events(|event| match event
        {
            Event::WindowEvent {
                window_id, event
            } => {
                match event {
                    WindowEvent::CloseRequested => {*running = false},
                    WindowEvent::KeyboardInput{device_id, input} =>{
                        engine::input::Input::get_instance().borrow_mut().on_keyboard_event(input);
                    },
                    _ => {}
                }
            },
            _ => {}
        });

    }

    pub fn stop(&self){
    }

    pub fn running(&self) -> bool{
        self.running
    }
}

