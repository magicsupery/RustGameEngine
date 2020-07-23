#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

extern crate gfx_backend_vulkan as back;

use gfx_hal::{buffer, command, format as f, format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle}, image as i, memory as m, pass, pass::Subpass, pool, prelude::*, pso, pso::{PipelineStage, ShaderStageFlags, VertexInputRate}, queue::{QueueGroup, Submission}, window, Instance};

use std::{
    borrow::Borrow,
    io::Cursor,
    iter,
    mem::{self, ManuallyDrop},
    ptr,
};
use gfx_hal::device::Device;
use gfx_hal::window::PresentationSurface;

pub(crate) struct Render<B: gfx_hal::Backend> {
    instance: Option<B::Instance>,
    surface: ManuallyDrop<B::Surface>,
    adapter: gfx_hal::adapter::Adapter<B>,
}

impl<B> Render<B>
    where
        B: gfx_hal::Backend,
{
    pub fn new(
        instance: Option<B::Instance>,
        mut surface: B::Surface,
        adapter: gfx_hal::adapter::Adapter<B>,
    ) -> Render<B> {
        Render{
            instance,
            surface: ManuallyDrop::new(surface),
            adapter
        }
    }
}

