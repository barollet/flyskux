use std::sync::Arc;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::Device;

use crate::engine::Engine;
use super::Vertex;

pub struct Triangle {
    vertices: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

impl Triangle {
    pub fn from_vertices(device: Arc<Device>, vertices: [Vertex; 3]) -> Self {
        Triangle {
            vertices: CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::all(),
                vertices.iter().cloned(),
            )
            .unwrap(),
        }
    }

    pub fn vertex_buffer(&self) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        self.vertices.clone()
    }
}

impl Engine {
    pub fn new_triangle(&mut self, vertices: [[f32; 2]; 3]) {
        self.renderables.push(Triangle::from_vertices(
            self.device.clone(),
            [
                Vertex {
                    position: vertices[0],
                },
                Vertex {
                    position: vertices[1],
                },
                Vertex {
                    position: vertices[2],
                },
            ],
        ));
    }
}
