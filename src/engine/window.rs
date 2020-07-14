extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;

use glutin::GlContext;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct Window{
    event_loop :  glutin::EventsLoop,
    window : glutin::GlWindow,
    //device : Option<dyn Device>,
    running : bool,
}

pub fn create_window(width: u32, height: u32, title: &str) -> Window {
    let window_builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height);

    let context_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
        .with_vsync(true);

    let event_loop = glutin::EventsLoop::new();
    let (win, _device, mut _factory, _color_view, mut _depth_view) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(
            window_builder, context_builder, &event_loop);

    let window = Window {
        event_loop,
        window: win,
        //device: None,
        running: true,
    };

    window

}


impl Window{
    pub fn render(&mut self){
        let running = &mut self.running;
        self.event_loop.poll_events(|event|{
            if let glutin::Event::WindowEvent {event, ..} = event{
                match event {
                    glutin::WindowEvent::Closed =>*running = false,
                    _ => {}
                }
            }
        });

    }

    pub fn stop(&self){
        self.window.swap_buffers().unwrap();
        //self.device.cleanup();
    }

    pub fn running(&self) -> bool{
        self.running
    }
}

