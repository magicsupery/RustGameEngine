#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use gfx_hal::{buffer, command, format as f, format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle}, image as i, memory as m, pass, pass::Subpass, pool, prelude::*, pso, pso::{PipelineStage, ShaderStageFlags, VertexInputRate}, queue::{QueueGroup, Submission}, window, Instance, Backend};

use std::{
    borrow::Borrow,
    io::Cursor,
    iter,
    mem::{self, ManuallyDrop},
    ptr,
};
use gfx_hal::device::Device;
use gfx_hal::window::PresentationSurface;

use gfx_backend_vulkan as back;

pub struct Render {
    instance: Option<<back::Backend as Backend>::Instance>,
    surface: ManuallyDrop<<back::Backend as Backend>::Surface>,
    adapter: gfx_hal::adapter::Adapter<back::Backend>,
    device : <back::Backend as Backend>::Device,
}

impl Render
{
    pub fn new(
        window: &winit::window::Window
    ) -> Render {

        let (instance, mut adapters, surface) = {
            let instance =
                back::Instance::create("GameEngine", 1).expect("Failed to create an instance!");
            let surface = unsafe {
                instance
                    .create_surface(window)
                    .expect("Failed to create a surface!")
            };
            let adapters = instance.enumerate_adapters();
            // Return `window` so it is not dropped: dropping it invalidates `surface`.
            (Some(instance), adapters, surface)
        };

        let adapter = adapters.remove(0);

        // craete device
        let family = adapter.queue_families.iter()
            .find(|family| {
            surface.supports_queue_family(family) && family.queue_type().supports_graphics()
        }).unwrap();

        let mut gpu = unsafe {
            adapter.physical_device
                .open(&[(family, &[1.0])], gfx_hal::Features::empty())
        }.unwrap();


        let queue_group = gpu.queue_groups.pop().unwrap();
        let mut command_pool = unsafe {
            gpu.device.create_command_pool(queue_group.family, pool::CommandPoolCreateFlags::empty())
        };

        Render{
            instance,
            surface: ManuallyDrop::new(surface),
            adapter,
            device: gpu.device,
        }
    }

    pub fn render(&mut self){

    }
}

