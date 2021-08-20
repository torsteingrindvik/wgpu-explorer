use std::{borrow::Cow, mem};

use crate::{square::Square, texture_image::TextureImage, vertex::Vertex, viewport::Viewport};
use color_eyre::Result;
use wgpu::*;
use winit::event::VirtualKeyCode;

pub struct WindowExtra {
    pub viewport: Viewport,
    pub render_pipeline: RenderPipeline,
    pub left_bind_group: BindGroup,
    pub right_bind_group: BindGroup,
    pub left_image: TextureImage,
    pub right_image: TextureImage,
    pub left_square: Square,
    pub right_square: Square,
}

fn bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Extra window bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                count: None,
            },
        ],
    })
}

fn bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    texture_view: &TextureView,
    sampler: &Sampler,
) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Extra window bind group"),
        layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(sampler),
            },
        ],
    })
}

fn pipeline_layout(device: &Device, bind_group_layout: &BindGroupLayout) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Extra window pipeline layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn render_pipeline(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    format: &TextureFormat,
) -> RenderPipeline {
    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: Some("Extra shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/extra.wgsl"))),
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Extra render pipeline"),
        layout: Some(pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[VertexBufferLayout {
                array_stride: mem::size_of::<Vertex>() as BufferAddress,
                step_mode: VertexStepMode::Vertex,
                attributes: &[
                    VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: mem::size_of::<[f32; 2]>() as u64,
                        shader_location: 1,
                    },
                ],
            }],
        },
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[format.to_owned().into()],
        }),
        multisample: MultisampleState::default(),
    })
}

fn sampler(device: &Device) -> Sampler {
    device.create_sampler(&SamplerDescriptor {
        label: Some("Extra sampler"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
    })
}

impl WindowExtra {
    pub fn new(
        viewport: Viewport,
        device: &Device,
        queue: &Queue,
        texture_format: &TextureFormat,
    ) -> Result<Self> {
        let layout = bind_group_layout(device);

        let left_image = TextureImage::new_from_path(
            "aztec diffuse image",
            device,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/aztec-diffuse.png"),
        )?;

        let right_image = TextureImage::new_from_path(
            "aztec height image",
            device,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/aztec-height.png"),
        )?;

        let sampler = sampler(device);

        let left_bind_group = bind_group(device, &layout, &left_image.texture_view, &sampler);

        let right_bind_group = bind_group(device, &layout, &right_image.texture_view, &sampler);

        let pipeline_layout = pipeline_layout(device, &layout);
        let render_pipeline = render_pipeline(device, &pipeline_layout, texture_format);

        let left_square = Square::new_from_vertices([
            Vertex::new(-1.0, 1.0, 0.0, 0.0),
            Vertex::new(0.0, 1.0, 1.0, 0.0),
            Vertex::new(0.0, -1.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, 0.0, 1.0),
        ]);

        let right_square = Square::new_from_vertices([
            Vertex::new(0.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 1.0, 0.0),
            Vertex::new(1.0, -1.0, 1.0, 1.0),
            Vertex::new(0.0, -1.0, 0.0, 1.0),
        ]);

        let new_self = Self {
            viewport,
            render_pipeline,
            left_bind_group,
            right_bind_group,
            left_image,
            right_image,
            left_square,
            right_square,
        };

        new_self.push_resources(device, queue)?;

        Ok(new_self)
    }

    pub fn handle_key(&mut self, _key: VirtualKeyCode) {
        // use winit::event::VirtualKeyCode::*;
        // match key {
        //     _ => {}
        // }

        // self.viewport.window.request_redraw();
    }

    fn push_resources(&self, _device: &Device, queue: &Queue) -> Result<()> {
        self.left_image.write(queue);
        self.right_image.write(queue);

        Ok(())
    }

    fn render_extra(
        &self,
        square: &Square,
        bind_group: &BindGroup,
        device: &Device,
        encoder: &mut CommandEncoder,
        texture_view: &TextureView,
    ) {
        let index_buffer = square.index_buffer(device);
        let vertex_buffer = square.vertex_buffer(device);

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Extra render pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // TODO: Check out debug group, debug marker calls etc.
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, bind_group, &[]);
            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.draw_indexed(0..square.indices.len() as u32, 0, 0..1);
        }
    }

    pub fn render(&self, device: &Device, queue: &Queue) -> Result<()> {
        let surface_texture = self.viewport.surface.get_current_frame()?.output;
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Extra command encoder"),
        });

        self.render_extra(
            &self.left_square,
            &self.left_bind_group,
            device,
            &mut encoder,
            &texture_view,
        );

        self.render_extra(
            &self.right_square,
            &self.right_bind_group,
            device,
            &mut encoder,
            &texture_view,
        );

        queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
