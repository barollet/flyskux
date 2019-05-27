mod triangle;

pub use triangle::*;

use super::Engine;

impl Engine {
    pub fn push_triangle(&mut self) {
        self.renderables.push(Triangle::from_vertices(
            self.device.clone(),
            [
                Vertex {
                    position: [-0.5, -0.25],
                },
                Vertex {
                    position: [0.0, 0.5],
                },
                Vertex {
                    position: [0.25, -0.1],
                },
            ],
        ));
    }
}
