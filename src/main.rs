use winit::{Event, EventsLoop, WindowEvent};

extern crate engine;
mod game;

use engine::Engine;

pub struct Application {
    // Window management
    events_loop: EventsLoop,

    // Vulkan management
    engine: Engine,
}

impl Application {
    fn init() -> Self {
        let events_loop = EventsLoop::new();

        Self {
            engine: Engine::init(&events_loop),
            events_loop,
        }
    }

    // Application main loop
    fn main_loop(&mut self) {
        loop {
            let mut done = false;

            self.engine.rendering_system.render();

            self.events_loop.poll_events(|ev| {
                if let Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } = ev
                {
                    done = true
                }
            });
            if done {
                return;
            }
        }
    }
}

fn main() {
    let mut flyskux = Application::init();
    flyskux.engine.rendering_system.add_sprite_component(&[[0.25, 0.25], [0.5, 0.5], [0.25, 0.5], [-0.25, -0.5], [-0.25, 0.25], [-0.5, -0.5]]);
    flyskux.engine.rendering_system.add_sprite_component(&[[-0.25, 0.25], [-0.5, -0.5], [0.25, 0.5]]);
    flyskux.main_loop();
}
