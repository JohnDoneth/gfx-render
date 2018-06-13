
extern crate gfx_render;

//use gfx_render::init;

#[macro_use]
extern crate log;

use gfx_render::vulkan;

extern crate simple_logger;

extern crate winit;

extern crate gfx_hal as hal;

use gfx_render::{Render, BackendEx, Factory};

use std::collections::HashMap;


use hal::{
    buffer, command::{
        ClearValue, ClearColor, ClearDepthStencil, CommandBuffer, Primary, RenderPassInlineEncoder,
        DescriptorSetOffset,
    },
    device::WaitFor,
    format::{ChannelType, Format}, image, image::{Extent, StorageFlags, Tiling},
    memory::{cast_slice, Barrier, Dependencies, Pod, Properties},
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        AllocationError, Descriptor, DescriptorPool, DescriptorRangeDesc,
        DescriptorSetLayoutBinding, DescriptorSetWrite, DescriptorType, ElemStride, Element,
        EntryPoint, GraphicsShaderSet, PipelineStage, Rect, ShaderStageFlags, VertexBufferSet,
        Viewport, ColorBlendDesc, ColorMask, BlendState, DepthTest, Comparison, StencilTest, DepthStencilDesc,
    },
    queue::{Graphics, QueueFamilyId},
    window::{Backbuffer, Extent2D, FrameSync, Swapchain, SwapchainConfig}, Backend, Device,
    Instance, PhysicalDevice, Surface,
};

#[derive(Clone)]
struct RenderData {



}

unsafe impl Send for RenderData {}
unsafe impl Sync for RenderData {}

impl<B : BackendEx> Render<B, Self> for RenderData {

    fn run(
        &mut self,
        fences: &mut Vec<B::Fence>,
        queues: &mut HashMap<QueueFamilyId, Vec<B::CommandQueue>>,
        factory: &mut Factory<B>,
        data: &mut Self,
    ) -> usize {

        CommandBuffer

        0usize
    }

    fn dispose(self, factory: &mut Factory<B>, data: &mut Self) -> Backbuffer<B> {

        unimplemented!();

    }

}


fn main() {

    simple_logger::init().unwrap();

    let mut events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("gfx-render - Triangle Example")
        .build(&events_loop)
        .unwrap();

    let (mut factory, mut renderer) = gfx_render::init::<vulkan::Backend, RenderData, _>(|queues|{

        for queue in queues.iter() {
            println!("{:?}", queue);
        }

        return vec!((&queues[0], 1));

    }).unwrap();

    let mut render_data = RenderData { };

    let target_id = renderer.add_target(&window, &mut factory);

    renderer.set_render(
        target_id, 
        &mut factory, 
        &mut render_data, 
        |surface, queues, factory, render_data| -> Result<_, ::std::io::Error> {

            let (capabilites, formats) = factory.capabilities_and_formats(&surface);
            let surface_format = formats.map_or(Format::Rgba8Srgb, |formats| {
                info!("Surface formats: {:#?}", formats);
                formats
                    .iter()
                    .find(|&format| format.base_format().1 == ChannelType::Srgb)
                    .cloned()
                    .unwrap_or(formats[0])
            });
            info!("Chosen surface format: {:#?}", surface_format);

            let extent = surface.kind().extent();

            info!("Extent: {:#?}", extent);


            Ok( RenderData {} )

        }).expect("Failed to set_render");

    loop {

        events_loop.poll_events(|event| {
            println!("{:?}", event);

            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => {
                    
                }
                _ => {}
            }
        });

        //let mut data = RenderData {};
        //renderer.run(&mut factory, &mut data);

    }

}