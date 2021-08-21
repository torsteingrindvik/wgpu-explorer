use std::mem;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupLayoutEntry, BindingType, BufferAddress, BufferBindingType, BufferDescriptor,
    BufferSize, BufferUsages, ShaderStages,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ResolutionUniform {
    /// The resolution of the window,
    pub resolution: [f32; 2],
}

impl ResolutionUniform {
    pub const fn size(&self) -> BufferAddress {
        mem::size_of::<ResolutionUniform>() as BufferAddress
    }

    pub const fn bind_group_layout_entry(&self, binding: u32) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::all(),
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
            label: Some("Resolution"),
            size: self.size(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.resolution = [size.width as f32, size.height as f32];
    }
}
