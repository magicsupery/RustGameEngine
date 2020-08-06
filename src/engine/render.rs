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
use gfx_hal::window::{PresentationSurface, SwapchainConfig, Suboptimal};

use gfx_backend_vulkan as back;
use gfx_hal::format::{Format, Aspects};
use gfx_hal::image::{Usage, Layout, ViewKind, SubresourceRange, Extent};
use gfx_hal::adapter::Gpu;
use gfx_hal::pass::{Attachment, AttachmentOps, AttachmentLoadOp, AttachmentStoreOp, SubpassDesc};
use gfx_hal::pool::{CommandPoolCreateFlags, CommandPool};
use gfx_hal::command::CommandBuffer;
use gfx_hal::pso::Rect;
use arrayvec::ArrayVec;

pub struct Render {
    instance: Option<<back::Backend as Backend>::Instance>,
    surface: ManuallyDrop<<back::Backend as Backend>::Surface>,
    adapter: gfx_hal::adapter::Adapter<back::Backend>,
    device : <back::Backend as Backend>::Device,
    queue_group: QueueGroup<back::Backend>,
    swapchain: ManuallyDrop<<back::Backend as Backend>::Swapchain>,

    render_area: Rect,
    render_pass: ManuallyDrop<<back::Backend as Backend>::RenderPass>,
    image_views: Vec<(<back::Backend as Backend>::ImageView)>,
    frame_buffers: Vec<<back::Backend as Backend>::Framebuffer>,
    command_pool: ManuallyDrop<<back::Backend as Backend>::CommandPool>,
    command_buffers: Vec<<back::Backend as Backend>::CommandBuffer>,
    image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore>,
    render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore>,
    in_flight_fences: Vec<(<back::Backend as Backend>::Fence)>,
    frames_in_flight: usize,
    current_frame: usize,
}

impl Render
{
    pub fn new(
        window: &winit::window::Window
    ) -> Result<Self, &'static str>{
        let (instance, mut adapters, mut surface) = {
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

        // Open A Device and take out a QueueGroup
        let (device, queue_group) = {
            let queue_family = adapter
                .queue_families
                .iter()
                .find(|qf| qf.queue_type().supports_graphics() && surface.supports_queue_family(qf))
                .ok_or("Couldn't find a QueueFamily with graphics!")?;

            let Gpu { device, mut queue_groups } = unsafe {
                adapter
                    .physical_device
                    .open(&[(&queue_family, &[1.0; 1])], gfx_hal::Features::empty())
                    .map_err(|_| "Couldn't open the PhysicalDevice!")?
            };
            let queue_group = queue_groups.pop().unwrap();

            if queue_group.queues.len() > 0 {
                Ok(())
            } else {
                Err("The QueueGroup did not have any CommandQueues available!")
            }?;

            (device, queue_group)
        };

        //create swapchain
        let (swapchain, extent, images, format, frames_in_flight) = {
            let cap = surface.capabilities(&adapter.physical_device);
            let present_modes = cap.present_modes;
            let composite_alpha_modes = cap.composite_alpha_modes;
            info!("present_modes is {:?}", present_modes);

            use gfx_hal::window::PresentMode;
            let present_mode = {
                [PresentMode::IMMEDIATE, PresentMode::MAILBOX, PresentMode::FIFO, PresentMode::RELAXED]
                    .iter()
                    .cloned()
                    .find(|m| present_modes.contains(*m))
                    .ok_or("No PresentMode found")?
            };
            let composite_alpha = {
                use gfx_hal::window::CompositeAlphaMode;
                [CompositeAlphaMode::OPAQUE, CompositeAlphaMode::INHERIT,
                    CompositeAlphaMode::POSTMULTIPLIED, CompositeAlphaMode::PREMULTIPLIED]
                    .iter()
                    .cloned()
                    .find(|ca| composite_alpha_modes.contains(*ca))
                    .ok_or("No CompositeAlpha values specified!")?
            };

            let formats = surface.supported_formats(&adapter.physical_device);
            let format = match formats{
                None => Format::Rgba8Srgb,
                Some(formats) => match formats
                    .iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .cloned()
                {
                    Some(srgb_format) => srgb_format,
                    None => formats
                        .get(0)
                        .cloned()
                        .ok_or("Preferred format list was empty!")?,
                },
            };
            let extent = *cap.extents.end();
            let image_count = if present_mode == PresentMode::MAILBOX{
                let max_num : u32 = 3;
                (cap.image_count.end() - 1).min(*cap.image_count.start().max(&max_num))
            } else {
                let max_num  :u32 = 2;
                (cap.image_count.end() - 1).min(*cap.image_count.start().max(&max_num))
            };

            let image_layers = 1;
            let image_usage = if cap.usage.contains(Usage::COLOR_ATTACHMENT) {
                Usage::COLOR_ATTACHMENT
            } else {
                Err("The Surface isn't capable of supporting color!")?
            };
            let swapchain_config = SwapchainConfig {
                present_mode,
                composite_alpha_mode: composite_alpha,
                format,
                extent,
                image_count,
                image_layers,
                image_usage,
            };
            info!("{:?}", swapchain_config);
            //
            let (swapchain, images) = unsafe {
                device
                    .create_swapchain(&mut surface, swapchain_config, None)
                    .map_err(|_| "Failed to create the swapchain!")?
            };
            (swapchain, extent, images, format, image_count as usize)
        };

        // Create Our Sync Primitives
        let (image_available_semaphores, render_finished_semaphores, in_flight_fences) = {
            let mut image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
            let mut render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
            let mut in_flight_fences: Vec<<back::Backend as Backend>::Fence> = vec![];
            for _ in 0..frames_in_flight {
                in_flight_fences.push(
                    device
                        .create_fence(true)
                        .map_err(|_| "Could not create a fence!")?,
                );
                image_available_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create a semaphore!")?,
                );
                render_finished_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create a semaphore!")?,
                );
            }
            (
                image_available_semaphores,
                render_finished_semaphores,
                in_flight_fences,
            )
        };

        // Define A RenderPass
        let render_pass = {
            let color_attachment = Attachment {
                format: Some(format),
                samples: 1,
                ops: AttachmentOps {
                    load: AttachmentLoadOp::Clear,
                    store: AttachmentStoreOp::Store,
                },
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };
            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };
            unsafe {
                device
                    .create_render_pass(&[color_attachment], &[subpass], &[])
                    .map_err(|_| "Couldn't create a render pass!")?
            }
        };

        // Create The ImageViews
        let image_views: Vec<_> = images
            .into_iter()
            .map(|image| unsafe {
                device
                    .create_image_view(
                        &image,
                        ViewKind::D2,
                        format,
                        Swizzle::NO,
                        SubresourceRange {
                            aspects: Aspects::COLOR,
                            levels: 0..1,
                            layers: 0..1,
                        },
                    )
                    .map_err(|_| "Couldn't create the image_view for the image!")
            })
            .collect::<Result<Vec<_>, &str>>()?;

        // Create Our FrameBuffers
        let frame_buffers: Vec<<back::Backend as Backend>::Framebuffer> = {
            image_views
                .iter()
                .map(|image_view| unsafe {
                    device
                        .create_framebuffer(
                            &render_pass,
                            vec![image_view],
                            Extent {
                                width: extent.width as u32,
                                height: extent.height as u32,
                                depth: 1,
                            },
                        )
                        .map_err(|_| "Failed to create a framebuffer!")
                })
                .collect::<Result<Vec<_>, &str>>()?
        };

        // Create Our CommandPool
        let mut command_pool = unsafe {
            device
                .create_command_pool(queue_group.family, CommandPoolCreateFlags::RESET_INDIVIDUAL)
                .map_err(|_| "Could not create the raw command pool!")?
        };

        // Create Our CommandBuffers
        let command_buffers: Vec<_> = frame_buffers
            .iter()
            .map(|_| unsafe{command_pool.allocate_one(gfx_hal::command::Level::Primary)})
            .collect();

        Ok(Self{
            instance,
            surface: ManuallyDrop::new(surface),
            adapter,
            device,
            queue_group,
            swapchain: ManuallyDrop::new(swapchain),
            render_area: extent.to_extent().rect(),
            render_pass: ManuallyDrop::new(render_pass),
            image_views,
            frame_buffers,
            command_pool: ManuallyDrop::new(command_pool),
            command_buffers,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            frames_in_flight,
            current_frame: 0,
        })
    }

    pub fn render(&mut self) -> Result<(), &'static str> {
        // SETUP FOR THIS FRAME
        let image_available = &self.image_available_semaphores[self.current_frame];
        let render_finished = &self.render_finished_semaphores[self.current_frame];
        // Advance the frame _before_ we start using the `?` operator
        self.current_frame = (self.current_frame + 1) % self.frames_in_flight;

        let (i_u32, i_usize) = unsafe {
            let image_index = self
                .swapchain
                .acquire_image(core::u64::MAX, Option::from(image_available), None)
                .map_err(|_| "Couldn't acquire an image from the swapchain!")?;

            let size:usize = image_index.0 as usize;
            (image_index, size)
        };

        let flight_fence = &self.in_flight_fences[i_usize];
        unsafe {
            self.device
                .wait_for_fence(flight_fence, core::u64::MAX)
                .map_err(|_| "Failed to wait on the fence!")?;
            self.device
                .reset_fence(flight_fence)
                .map_err(|_| "Couldn't reset the fence!")?;
        }

        // RECORD COMMANDS
        unsafe {
            let buffer = &mut self.command_buffers[i_usize];
            let clear_values =
                [command::ClearValue{color: command::ClearColor{float32 : [0.0, 0.0, 0.0, 0.0]}}];
            buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);
            buffer.begin_render_pass(
                &self.render_pass,
                &self.frame_buffers[i_usize],
                self.render_area,
                clear_values.iter(),
                command::SubpassContents::Inline
            );
            buffer.finish();
        }

        // SUBMISSION AND PRESENT
        let command_buffers = &self.command_buffers[i_usize..=i_usize];
        let wait_semaphores: ArrayVec<[_; 1]> =
            [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        // yes, you have to write it twice like this. yes, it's silly.
        let present_wait_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };
        let the_command_queue = &mut self.queue_group.queues[0];
        unsafe {
            the_command_queue.submit(submission, Some(flight_fence));
            self.swapchain
                .present(the_command_queue, i_u32.0, present_wait_semaphores)
                .map_err(|_| "Failed to present into the swapchain!")
        };

        Ok(())
    }
}

