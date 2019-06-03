// Entry point of the Vulkan engine

// Initialize Vulkan instance, device and swapchain
// The main window is initialized with the swapchain creation

pub mod rendering;
mod shaders;

use std::sync::Arc;

use winit::EventsLoop;

use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;

use rendering::*;

pub struct Engine {
    device: Arc<Device>,
    pub rendering_system: RenderingSystem,
}

impl Engine {
    // initializing the main engine
    pub fn init(events_loop: &EventsLoop) -> Self {
        let instance = init_instance();
        let (device, queue) = init_device(instance.clone());

        Self {
            rendering_system: RenderingSystem::init(instance.clone(), device.clone(), queue.clone(), &events_loop),
            device,
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
