use std::mem;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupLayoutEntry, BindingType, BufferAddress, BufferBindingType, BufferDescriptor,
    BufferSize, BufferUsages, ShaderStages,
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RadarUniform {
    /// The direction the radar is looking,
    /// (x, y) in ranges [-1, 1]
    pub view_dir: [f32; 2],

    /// The position of the radar,
    /// (x, y) in ranges [-1, 1]
    pub position: [f32; 2],

    /// The field of view of the radar in radians
    // pub fov: f32,
    pub fov: [f32; 2],
}

impl RadarUniform {
    pub const fn size(&self) -> BufferAddress {
        mem::size_of::<RadarUniform>() as BufferAddress
    }

    pub const fn bind_group_layout_entry(&self, binding: u32) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(self.size()),
            },
            count: None,
        }
    }

    pub fn buffer_descriptor(&self) -> BufferDescriptor {
        BufferDescriptor {
            label: Some("Radar"),
            size: self.size(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }
    }
}
