use std::rc::Rc;
use std::cell::RefCell;

use winit::event::{Event, WindowEvent};
type KeyboardInput = winit::event::KeyboardInput;
type KeyboardEventCallback = fn(&KeyboardInput);

pub struct Input{
   key_pressed_callbacks: Vec<KeyboardEventCallback>,
}

type InputSptr = Rc<RefCell<Input>>;

impl Input {
   pub fn get_instance() -> &'static mut InputSptr {
      static mut INSTANCE: Option<InputSptr> = None;

      unsafe {
         INSTANCE.get_or_insert_with(|| -> InputSptr {
            Rc::new(RefCell::new(Input{ key_pressed_callbacks: vec![] }))
         })
      }
   }

   pub fn on_keyboard_event(&mut self, input :KeyboardInput){
      println!("{:?}", input);
      match input.state{
         winit::event::ElementState::Pressed => {
            for cb in self.key_pressed_callbacks.iter() {
               cb(&input);
            }
         },
         winit::event::ElementState::Released => {

         },
      }
   }
}
