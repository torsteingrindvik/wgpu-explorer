use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsage, Device,
};

use crate::{camera::Camera, vec::Vec2, vertex::Vertex};

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub indices: [u16; 3],
}

impl From<&Camera> for Triangle {
    fn from(camera: &Camera) -> Self {
        // let v1 = Vertex::new(camera.x, camera.y);

        let x1 = camera.viewing_dir.x * camera.viewing_distance;
        let y1 = camera.viewing_dir.y * camera.viewing_distance;

        let first_rot = camera.fov / 2.0;
        let second_rot = std::f32::consts::TAU - first_rot;

        let x2 = first_rot.cos() * x1 - first_rot.sin() * y1;
        let y2 = first_rot.sin() * x1 + first_rot.cos() * y1;

        let x3 = second_rot.cos() * x1 - second_rot.sin() * y1;
        let y3 = second_rot.sin() * x1 + second_rot.cos() * y1;

        // let half_fow = camera.fov / 2.0;
        // let abs_angle_max = camera.viewing_angle + half_fow;
        // let abs_angle_min = camera.viewing_angle - half_fow;

        // let v2_x = abs_angle_max.cos() * camera.viewing_distance;
        // let v2_y = abs_angle_max.sin() * camera.viewing_distance;
        // let v2 = Vertex::new(camera.x + v2_x, camera.y + v2_y);

        // let v3_x = abs_angle_min.cos() * camera.viewing_distance;
        // let v3_y = abs_angle_min.sin() * camera.viewing_distance;
        // let v3 = Vertex::new(camera.x + v3_x, camera.y + v3_y);

        // v2_x = abs_angle_max.cos() *

        Self {
            vertices: [
                Vertex::new(x1 + camera.x, y1 + camera.y),
                Vertex::new(x2 + camera.x, y2 + camera.y),
                Vertex::new(x3 + camera.x, y3 + camera.y),
            ],
            indices: [0, 1, 2],
        }
    }
}

impl Triangle {
    /// Given vertices in clockwise order, sets up the rest.
    // pub fn new_from_vertices(vertices: [Vertex; 3]) -> Self {
    //     Triangle::new(vertices, [0, 1, 2])
    // }

    // pub fn new(vertices: [Vertex; 3], indices: [u16; 3]) -> Self {
    //     Self { vertices, indices }
    // }

    // pub fn new(x: f32, y: f32)

    // pub fn displace(&mut self, direction: Direction, amount: f32) {
    //     let (x, y) = misc::displace(direction, amount);

    // }
    pub fn center_at(&mut self, x: f32, y: f32) {
        let vec_to = Vec2::new_from_points(self.vertices[0].pos[0], self.vertices[0].pos[1], x, y);

        self.vertices[0].pos[0] += vec_to.x;
        self.vertices[0].pos[1] += vec_to.y;
        self.vertices[1].pos[0] += vec_to.x;
        self.vertices[1].pos[1] += vec_to.y;
        self.vertices[2].pos[0] += vec_to.x;
        self.vertices[2].pos[1] += vec_to.y;
    }

    pub fn vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer :)"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: BufferUsage::VERTEX,
        })
    }

    pub fn index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index buffer :)"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: BufferUsage::INDEX,
        })
    }
}

// impl Default for Triangle {
//     fn default() -> Self {
//         Self::new_from_vertices([
//             Vertex::new(-1.0, 0.0),
//             Vertex::new(0.0, 1.0),
//             Vertex::new(1.0, 0.0),
//         ])
//     }
// }
