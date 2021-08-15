use std::{borrow::Cow, mem};

use crate::{
    misc::Direction,
    square::Square,
    texture_image::TextureImage,
    vertex::{Vertex, VertexSelected},
    viewport::Viewport,
};
use color_eyre::Result;
use wgpu::*;
use winit::event::VirtualKeyCode;

const DIFF: f32 = 0.01;

pub struct WindowMain {
    pub viewport: Viewport,
    pub square: Square,
    pub render_pipeline: RenderPipeline,
    pub bind_group: BindGroup,
    pub image: TextureImage,
    pub displace_amount: f32,
}

fn bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStage::FRAGMENT,
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
        label: Some("Main window bind group"),
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
        label: None,
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
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        flags: ShaderFlags::default(),
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[VertexBufferLayout {
                array_stride: mem::size_of::<Vertex>() as BufferAddress,
                step_mode: InputStepMode::Vertex,
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

impl WindowMain {
    pub fn new(
        viewport: Viewport,
        device: &Device,
        queue: &Queue,
        texture_format: &TextureFormat,
    ) -> Result<Self> {
        let bind_group_layout = bind_group_layout(device);

        let image_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/rocks.png");

        let image = TextureImage::new(device, image_path)?;
        let bind_group = bind_group(
            device,
            &bind_group_layout,
            &image.texture_view,
            &image.sampler,
        );

        let pipeline_layout = pipeline_layout(device, &bind_group_layout);
        let render_pipeline = render_pipeline(device, &pipeline_layout, texture_format);

        let square = Square::default();

        // let square = Square::new_from_vertices([
        //     Vertex::new(-1.0, 1.0, 0.0, 0.0),
        //     Vertex::new(0.0, 1.0, 1.0, 0.0),
        //     Vertex::new(0.0, -1.0, 1.0, 1.0),
        //     Vertex::new(-1.0, -1.0, 0.0, 1.0),
        // ]);

        let displace_amount = 0.01;

        let new_self = Self {
            viewport,
            square,
            render_pipeline,
            bind_group,
            image,
            displace_amount,
        };

        new_self.push_resources(device, queue)?;

        Ok(new_self)
    }

    pub fn handle_key(&mut self, key: VirtualKeyCode) {
        use winit::event::VirtualKeyCode::*;
        match key {
            F1 => self.displace_amount = f32::max(DIFF, self.displace_amount - DIFF),
            F2 => self.displace_amount += DIFF,

            Key1 | Numpad1 => self.square.set_selected(VertexSelected::One),
            Key2 | Numpad2 => self.square.set_selected(VertexSelected::Two),
            Key3 | Numpad3 => self.square.set_selected(VertexSelected::Three),
            Key4 | Numpad4 => self.square.set_selected(VertexSelected::Four),

            Left | A => self.square.displace(Direction::Left, self.displace_amount),
            Right | D => self.square.displace(Direction::Right, self.displace_amount),
            Up | W => self.square.displace(Direction::Up, self.displace_amount),
            Down | S => self.square.displace(Direction::Down, self.displace_amount),

            _ => {}
        }

        self.viewport.window.request_redraw();
    }

    fn push_resources(&self, _device: &Device, queue: &Queue) -> Result<()> {
        self.image.write(queue);

        Ok(())
    }

    pub fn render(&self, device: &Device, queue: &Queue) -> Result<()> {
        let frame = self.viewport.swap_chain.get_current_frame()?.output;

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });

        let index_buffer = self.square.index_buffer(device);
        let vertex_buffer = self.square.vertex_buffer(device);

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // TODO: Check out debug group, debug marker calls etc.
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.draw_indexed(0..self.square.indices.len() as u32, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
