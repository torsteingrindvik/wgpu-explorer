use std::ops::{Index, IndexMut};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::{
    misc::{self, Direction},
    vertex::{Vertex, VertexSelected},
};

#[derive(Debug)]
pub struct Square {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
    pub selected: VertexSelected,
}

impl Square {
    /// Given vertices in clockwise order, sets up the rest.
    pub fn new_from_vertices(vertices: [Vertex; 4]) -> Self {
        Square::new(vertices, [0, 1, 3, 3, 1, 2], VertexSelected::One)
    }

    pub fn new(vertices: [Vertex; 4], indices: [u16; 6], selected: VertexSelected) -> Self {
        Self {
            vertices,
            indices,
            selected,
        }
    }

    pub fn displace(&mut self, direction: Direction, amount: f32) {
        let (x, y) = misc::displace(direction, amount);

        let selected = self.selected;

        self[selected].pos[0] += x;
        self[selected].pos[1] += y;
    }

    pub fn vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer :)"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: BufferUsages::VERTEX,
        })
    }

    pub fn index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index buffer :)"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: BufferUsages::INDEX,
        })
    }

    /// Set the square's selected.
    pub fn set_selected(&mut self, selected: VertexSelected) {
        self.selected = selected;
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new_from_vertices([
            Vertex::new(-0.8, 0.8, 0.0, 0.0),
            Vertex::new(0.8, 0.8, 1.0, 0.0),
            Vertex::new(0.8, -0.8, 1.0, 1.0),
            Vertex::new(-0.8, -0.8, 0.0, 1.0),
        ])
    }
}

impl Index<VertexSelected> for Square {
    type Output = Vertex;

    fn index(&self, vertex: VertexSelected) -> &Self::Output {
        match vertex {
            VertexSelected::One => &self.vertices[0],
            VertexSelected::Two => &self.vertices[1],
            VertexSelected::Three => &self.vertices[2],
            VertexSelected::Four => &self.vertices[3],
        }
    }
}

impl IndexMut<VertexSelected> for Square {
    fn index_mut(&mut self, vertex: VertexSelected) -> &mut Self::Output {
        match vertex {
            VertexSelected::One => &mut self.vertices[0],
            VertexSelected::Two => &mut self.vertices[1],
            VertexSelected::Three => &mut self.vertices[2],
            VertexSelected::Four => &mut self.vertices[3],
        }
    }
}
