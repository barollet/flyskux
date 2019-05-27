use std::sync::Arc;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::Device;

pub struct Triangle {
    vertices: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position);

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
