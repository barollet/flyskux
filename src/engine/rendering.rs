// Graphical pipeline

// Create the render pass
use std::sync::Arc;

use vulkano::device::Device;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::framebuffer::Subpass;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::swapchain::Swapchain;

use winit::Window;

use super::shaders::*;

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position);

// Initialize the render pass
pub fn init_render_pass(
    device: Arc<Device>,
    swapchain: Arc<Swapchain<Window>>,
) -> Arc<RenderPassAbstract> {
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
    render_pass: Arc<RenderPassAbstract>,
) -> Arc<GraphicsPipelineAbstract> {
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
