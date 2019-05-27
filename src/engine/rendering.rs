// Graphical pipeline

// Create the render pass
use std::sync::Arc;

use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::device::Device;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::framebuffer::Subpass;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::Swapchain;
use vulkano::sync;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;

use winit::Window;

use super::primitives::Vertex;
use super::shaders::*;

use super::Engine;

enum RenderingError {
    RecreateSwapchain,
    Ignore,
}

impl Engine {
    // Rendering loop to be called to update the screen
    pub fn render_loop(&mut self) {
        let clear_values = vec!([0.0, 0.0, 1.0, 1.0].into());
        let (image_num, acquire_future) =
            swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();

        let mut command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )
        .unwrap()
        // Before we can draw, we have to *enter a render pass*. There are two methods to do
        // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
        // not covered here.
        //
        // The third parameter builds the list of values to clear the attachments with. The API
        // is similar to the list of attachments when building the framebuffers, except that
        // only the attachments that use `load: Clear` appear in the list.
        .begin_render_pass(self.framebuffers[image_num].clone(), false, clear_values)
        .unwrap();
        // We are now inside the first subpass of the render pass. We add a draw command.
        //
        // The last two parameters contain the list of resources to pass to the shaders.
        // Since we used an `EmptyPipeline` object, the objects have to be `()`.
        for renderable in &self.renderables {
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

        // We leave the render pass by calling `draw_end`. Note that if we had multiple
        // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
        // next subpass.
        let command_buffer = command_buffer
            .end_render_pass()
            .unwrap()
            // Finish building the command buffer by calling `build`.
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
