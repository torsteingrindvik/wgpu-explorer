use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy)]
pub enum VertexSelected {
    One,
    Two,
    Three,
    Four,
}

impl Default for VertexSelected {
    fn default() -> Self {
        Self::One
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub tc: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, tx: f32, ty: f32) -> Self {
        Self {
            pos: [x, y],
            tc: [tx, ty],
        }
    }
}
