use std::{borrow::Cow, mem};

use crate::{
    camera::Camera,
    square::Square,
    texture_image::TextureImage,
    triangle::Triangle,
    vertex::{Vertex, VertexWithTextureCoords},
    viewport::Viewport,
};
use color_eyre::Result;
use wgpu::*;
use winit::event::VirtualKeyCode;

pub struct WindowExtra {
    pub viewport: Viewport,

    pub render_pipeline_square: RenderPipeline,
    pub render_pipeline_triangle: RenderPipeline,

    pub triangle_bind_group: BindGroup,
    pub left_bind_group: BindGroup,
    pub right_bind_group: BindGroup,

    pub left_image: TextureImage,
    pub right_image: TextureImage,
    pub left_square: Square,
    pub right_square: Square,
    // pub left_triangle: Triangle,
    // pub right_triangle: Triangle,
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

fn bind_group_squares(
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

fn bind_group_triangles(device: &Device) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Extra window triangle bind group"),
        layout: &device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        }),
        entries: &[],
    })
}

fn pipeline_layout_squares(device: &Device, bind_group_layout: &BindGroupLayout) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn pipeline_layout_triangles(device: &Device) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    })
}

fn render_pipeline_triangles(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    format: &TextureFormat,
) -> RenderPipeline {
    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("triangle.wgsl"))),
        flags: ShaderFlags::default(),
    });

    let mut format: ColorTargetState = format.to_owned().into();
    format.blend = Some(BlendState::ALPHA_BLENDING);

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[VertexBufferLayout {
                array_stride: mem::size_of::<Vertex>() as BufferAddress,
                step_mode: InputStepMode::Vertex,
                attributes: &[VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                }],
            }],
        },
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[format],
        }),
        multisample: MultisampleState::default(),
    })
}

fn render_pipeline_squares(
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
                array_stride: mem::size_of::<VertexWithTextureCoords>() as BufferAddress,
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

impl WindowExtra {
    pub fn new(
        viewport: Viewport,
        device: &Device,
        queue: &Queue,
        texture_format: &TextureFormat,
    ) -> Result<Self> {
        let bind_group_layout = bind_group_layout(device);

        let left_image = TextureImage::new_from_path(
            device,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/aztec-diffuse.png"),
        )?;
        let right_image = TextureImage::new_from_path(
            device,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/aztec-height.png"),
        )?;

        let left_bind_group = bind_group_squares(
            device,
            &bind_group_layout,
            &left_image.texture_view,
            &left_image.sampler,
        );

        let right_bind_group = bind_group_squares(
            device,
            &bind_group_layout,
            &right_image.texture_view,
            &right_image.sampler,
        );

        let triangle_bind_group = bind_group_triangles(device);

        let pipeline_layout_squares = pipeline_layout_squares(device, &bind_group_layout);
        let pipeline_layout_triangles = pipeline_layout_triangles(device);

        let render_pipeline_square =
            render_pipeline_squares(device, &pipeline_layout_squares, texture_format);
        let render_pipeline_triangle =
            render_pipeline_triangles(device, &pipeline_layout_triangles, texture_format);

        let left_square = Square::new_from_vertices([
            VertexWithTextureCoords::new(-1.0, 1.0, 0.0, 0.0),
            VertexWithTextureCoords::new(0.0, 1.0, 1.0, 0.0),
            VertexWithTextureCoords::new(0.0, -1.0, 1.0, 1.0),
            VertexWithTextureCoords::new(-1.0, -1.0, 0.0, 1.0),
        ]);

        // let right_square = Square::new_from_vertices([
        //     VertexWithTextureCoords::new(0.0, 1.0, 0.0, 0.0),
        //     VertexWithTextureCoords::new(1.0, 1.0, 1.0, 0.0),
        //     VertexWithTextureCoords::new(1.0, -1.0, 1.0, 1.0),
        //     VertexWithTextureCoords::new(0.0, -1.0, 0.0, 1.0),
        // ]);

        let right_square = Square::new_from_vertices([
            VertexWithTextureCoords::new(-1.0, 1.0, 0.0, 0.0),
            VertexWithTextureCoords::new(1.0, 1.0, 1.0, 0.0),
            VertexWithTextureCoords::new(1.0, -1.0, 1.0, 1.0),
            VertexWithTextureCoords::new(-1.0, -1.0, 0.0, 1.0),
        ]);


        // let left_triangle = Triangle::new_from_vertices([
        //     Vertex::new(-0.5, 0.0),
        //     Vertex::new(-0.4, 0.1),
        //     Vertex::new(-0.4, -0.1),
        // ]);

        // let right_triangle = Triangle::new_from_vertices([
        //     Vertex::new(0.5, 0.0),
        //     Vertex::new(0.6, 0.1),
        //     Vertex::new(0.6, -0.1),
        // ]);

        let new_self = Self {
            viewport,
            render_pipeline_square,
            render_pipeline_triangle,
            triangle_bind_group,
            left_bind_group,
            right_bind_group,
            left_image,
            right_image,
            left_square,
            right_square,
            // left_triangle,
            // right_triangle,
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

    fn render_square(
        &self,
        square: &Square,
        bind_group: &BindGroup,
        device: &Device,
        encoder: &mut CommandEncoder,
        frame: &SwapChainTexture,
    ) {
        let index_buffer = square.index_buffer(device);
        let vertex_buffer = square.vertex_buffer(device);

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // TODO: Check out debug group, debug marker calls etc.
            rpass.set_pipeline(&self.render_pipeline_square);
            rpass.set_bind_group(0, bind_group, &[]);
            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.draw_indexed(0..square.indices.len() as u32, 0, 0..1);
        }
    }

    // Try instancing it to the other thing too?
    fn render_triangle(
        &self,
        triangle: &Triangle,
        bind_group: &BindGroup,
        device: &Device,
        encoder: &mut CommandEncoder,
        frame: &SwapChainTexture,
    ) {
        let index_buffer = triangle.index_buffer(device);
        let vertex_buffer = triangle.vertex_buffer(device);

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // TODO: Check out debug group, debug marker calls etc.
            rpass.set_pipeline(&self.render_pipeline_triangle);
            rpass.set_bind_group(0, bind_group, &[]);
            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.draw_indexed(0..triangle.indices.len() as u32, 0, 0..1);
        }
    }

    pub fn render(&self, device: &Device, queue: &Queue, camera: &Camera) -> Result<()> {
        let frame = self.viewport.swap_chain.get_current_frame()?.output;

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });

        self.render_square(
            &self.left_square,
            &self.left_bind_group,
            device,
            &mut encoder,
            &frame,
        );

        self.render_square(
            &self.right_square,
            &self.right_bind_group,
            device,
            &mut encoder,
            &frame,
        );

        let mut left_triangle: Triangle = camera.into();
        left_triangle.center_at((camera.x / 2.0) - 0.5, camera.y);

        let mut right_triangle: Triangle = camera.into();
        right_triangle.center_at((camera.x / 2.0) + 0.5, camera.y);

        self.render_triangle(
            &left_triangle,
            &self.triangle_bind_group,
            device,
            &mut encoder,
            &frame,
        );

        self.render_triangle(
            &right_triangle,
            &self.triangle_bind_group,
            device,
            &mut encoder,
            &frame,
        );

        queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
