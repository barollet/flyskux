// Entry point of the Vulkan engine

// Initialize Vulkan instance, device and swapchain
// The main window is initialized with the swapchain creation

use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::{EventsLoop, Window, WindowBuilder};
use vulkano_win::VkSurfaceBuild;

use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;

use vulkano::image::swapchain::SwapchainImage;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Engine {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
}

impl Engine {
    // initializing the main engine
    pub fn init(events_loop: &EventsLoop) -> Self {
        let instance = init_instance();
        let (device, queue) = init_device(instance.clone());
        let (swapchain, images) =
            init_swapchain(instance.clone(), device.clone(), queue.clone(), events_loop);
        Self {
            instance,
            device,
            queue,
            swapchain,
            images,
        }
    }
}

// Initialize Vulkan main instance
fn init_instance() -> Arc<Instance> {
    let app_infos = app_info_from_cargo_toml!();

    let required_extensions = vulkano_win::required_extensions();
    Instance::new(Some(&app_infos), &required_extensions, None)
        .expect("failed to create Vulkan instance")
}

// Initialize Vulkan context
// This returns a single command queue but it may return multiple queues in the future
fn init_device(instance: Arc<Instance>) -> (Arc<Device>, Arc<Queue>) {
    // Finding a suitable physical device
    let physical_device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");
    // Finding a graphical queue family
    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    // Creating a device with the given queue family
    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical_device,
            physical_device.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    // Returning the first queue of the family
    (device, queues.next().unwrap())
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
