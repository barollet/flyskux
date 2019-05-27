mod triangle;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position);

pub use triangle::*;
