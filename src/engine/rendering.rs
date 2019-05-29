// Graphical pipeline

// Create the render pass
use winit::EventsLoop;
use winit::dpi::LogicalSize;
use winit::{Window, WindowBuilder};


use vulkano::instance::Instance;
use std::sync::Arc;
use vulkano_win::VkSurfaceBuild;

use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::framebuffer::Subpass;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::swapchain;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};
use vulkano::sync::GpuFuture;

use super::primitives::*;
use super::shaders::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct RenderingSystem {
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    graphical_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    dynamic_state: DynamicState,
}

impl RenderingSystem {
    pub fn init(instance: Arc<Instance>, device: Arc<Device>, queue: Arc<Queue>, events_loop: &EventsLoop) -> RenderingSystem {
        let (swapchain, images) =
            init_swapchain(instance.clone(), device.clone(), queue.clone(), events_loop);

        // Initialiazing the render pass
        let render_pass = init_render_pass(device.clone(), swapchain.clone());
        let graphical_pipeline = init_graphical_pipeline(device.clone(), render_pass.clone());

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        };
        let framebuffers = init_framebuffers(&images, render_pass.clone(), &mut dynamic_state);

        RenderingSystem {
            device,
            queue,
            swapchain,
            graphical_pipeline,
            framebuffers,
            dynamic_state,
        }
    }
}

// Initialize the main window with a Vulkan surface and a swapchain to draw on
fn init_swapchain(
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    events_loop: &EventsLoop,
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    // Instanciating the main window
    let surface = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
        .with_resizable(false)
        .build_vk_surface(events_loop, instance.clone())
        .expect("Failed to create window");
    let window = surface.window();

    // Swapchain parameters
    let caps = surface.capabilities(device.physical_device()).unwrap();
    let usage = caps.supported_usage_flags;
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    // Set the swapchain dimension to the window size.
    let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
        // convert to physical pixels
        let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
        [dimensions.0, dimensions.1]
    } else {
        panic!("Window no longer exists");
    };

    Swapchain::new(
        device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        initial_dimensions,
        1,
        usage,
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    )
    .unwrap()
}


impl RenderingSystem {
    // Rendering loop to be called to update the screen
    pub fn render(&mut self, renderables: &Vec<Triangle>) {
        let clear_values = vec![[0.0, 0.0, 1.0, 1.0].into()];
        let (image_num, acquire_future) =
            swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();

        let mut command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )
        .unwrap()
        .begin_render_pass(self.framebuffers[image_num].clone(), false, clear_values)
        .unwrap();

        for renderable in renderables {
            command_buffer = command_buffer
                .draw(
                    self.graphical_pipeline.clone(),
                    &self.dynamic_state,
                    vec![renderable.vertex_buffer().clone()],
                    (),
                    (),
                )
                .unwrap();
        }

        let command_buffer = command_buffer
            .end_render_pass()
            .unwrap()
            .build()
            .unwrap();

        let future = acquire_future
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();
    }
}

pub fn init_framebuffers(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

// Initialize the render pass
pub fn init_render_pass(
    device: Arc<Device>,
    swapchain: Arc<Swapchain<Window>>,
) -> Arc<RenderPassAbstract + Send + Sync> {
    Arc::new(
        vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    )
}

// Compile the graphical pipeline
pub fn init_graphical_pipeline(
    device: Arc<Device>,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
    let triangle_vert = basic_triangle_vert::Shader::load(device.clone()).unwrap();
    let triangle_frag = basic_triangle_frag::Shader::load(device.clone()).unwrap();

    Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(triangle_vert.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(triangle_frag.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    )
}
