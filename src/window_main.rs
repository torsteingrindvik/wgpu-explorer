use std::{
    borrow::Cow,
    f32::consts::FRAC_PI_3,
    mem,
    path::{Path, PathBuf},
};

use crate::{
    camera::Camera,
    misc::Direction,
    radar::{self, RadarUniform},
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
    pub camera: Camera,
    pub radar: RadarUniform,
    pub radar_buffer: Buffer,
    pub texture_format: TextureFormat,
    pub shader_path: PathBuf,
}

// fn sampler(device: &Device) -> Sampler {
//     device.create_sampler(&SamplerDescriptor {
//         label: Some("Main sampler"),
//         address_mode_u: AddressMode::ClampToEdge,
//         address_mode_v: AddressMode::ClampToEdge,
//         address_mode_w: AddressMode::ClampToEdge,
//         mag_filter: FilterMode::Nearest,
//         min_filter: FilterMode::Nearest,
//         mipmap_filter: FilterMode::Nearest,
//         ..Default::default()
//     })
// }

fn bind_group_layout(device: &Device, radar: &RadarUniform) -> BindGroupLayout {
    // device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    //     label: Some("Main bind group layout"),
    //     entries: &[
    //         BindGroupLayoutEntry {
    //             binding: 0,
    //             visibility: ShaderStages::FRAGMENT,
    //             ty: BindingType::Texture {
    //                 sample_type: TextureSampleType::Float { filterable: true },
    //                 view_dimension: TextureViewDimension::D2,
    //                 multisampled: false,
    //             },
    //             count: None,
    //         },
    //         BindGroupLayoutEntry {
    //             binding: 1,
    //             visibility: ShaderStages::FRAGMENT,
    //             ty: BindingType::Sampler {
    //                 filtering: true,
    //                 comparison: false,
    //             },
    //             count: None,
    //         },
    //     ],
    // })
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Radar layout"),
        entries: &[radar.bind_group_layout_entry(0)],
        // entries,
    })
}

fn bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    // texture_view: &TextureView,
    // sampler: &Sampler,
    radar: &RadarUniform,
) -> BindGroup {
    // device.create_bind_group(&BindGroupDescriptor {
    //     label: Some("Main window bind group"),
    //     layout,
    //     entries: &[
    //         BindGroupEntry {
    //             binding: 0,
    //             resource: BindingResource::TextureView(texture_view),
    //         },
    //         BindGroupEntry {
    //             binding: 1,
    //             resource: BindingResource::Sampler(sampler),
    //         },
    //     ],
    // })
    let radar_buffer = device.create_buffer(&radar.buffer_descriptor());
    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Radar bind group"),
        layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: radar_buffer.as_entire_binding(),
        }],
    })
}

fn pipeline_layout(device: &Device, bind_group_layout: &BindGroupLayout) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Main pipeline layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn render_pipeline(
    device: &Device,
    shader: &ShaderModule,
    pipeline_layout: &PipelineLayout,
    format: &TextureFormat,
) -> RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Main render pipeline"),
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

impl WindowMain {
    // fn rewrite_texture(&mut self) {
    //     let w = self.image.extent.width as usize;
    //     let h = self.image.extent.height as usize;

    //     debug!("w,h: {:?}", &(w, h));

    //     for x in 0..w {
    //         for y in 0..h {
    //             let widthf = w as f32;
    //             let heightf = h as f32;
    //             let xf = x as f32;
    //             let yf = y as f32;

    //             let xf = (-widthf + 2.0 * xf) / widthf;
    //             let yf = -(-heightf + 2.0 * yf) / heightf;

    //             if self.camera.within_view(xf, yf) {
    //                 self.image.set_pixel(x, y, Color::BLUE);
    //             } else {
    //                 self.image.set_pixel(x, y, Color::BLACK);
    //             }
    //         }
    //     }
    // }

    pub fn new(
        viewport: Viewport,
        device: &Device,
        texture_format: &TextureFormat,
    ) -> Result<Self> {
        // let bind_group_layout = bind_group_layout(device);
        let texture_format = *texture_format;
        let radar = radar::RadarUniform {
            fov: [FRAC_PI_3, 0.0],
            view_dir: [0.0, 1.0],
            position: [0.0, 0.0],
        };
        // let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        //     label: Some("Radar layout"),
        //     entries: &[radar.bind_group_layout_entry(0)],
        // });
        let bind_group_layout = bind_group_layout(device, &radar);

        let size = 512;
        let width = size;
        let height = size;

        // 4 bytes per point: rgba
        let data: Vec<u8> = vec![0; width * height * 4];
        let image = TextureImage::new("Main texture image", device, width, height, &data)?;
        let camera = Camera::default();

        let radar_buffer = device.create_buffer(&radar.buffer_descriptor());
        let bind_group = bind_group(device, &bind_group_layout, &radar);

        let pipeline_layout = pipeline_layout(device, &bind_group_layout);
        let shader_path = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/shaders/radar.wgsl"
        ));
        let shader = Self::load_shader(&shader_path, device)?;

        let render_pipeline = render_pipeline(device, &shader, &pipeline_layout, &texture_format);

        let square = Square::default();

        let displace_amount = 0.05;

        Ok(Self {
            viewport,
            square,
            render_pipeline,
            bind_group,
            image,
            displace_amount,
            camera,
            radar,
            radar_buffer,
            texture_format,
            shader_path,
        })
    }

    pub fn handle_key(&mut self, key: VirtualKeyCode) {
        use winit::event::VirtualKeyCode::*;
        match key {
            F1 => self.displace_amount = f32::max(DIFF, self.displace_amount - DIFF),
            F2 => self.displace_amount += DIFF,

            F3 => {
                self.camera.fov = f32::max(
                    std::f32::consts::FRAC_PI_8,
                    self.camera.fov - std::f32::consts::FRAC_PI_8 / 2.0,
                )
            }
            F4 => {
                self.camera.fov = f32::min(
                    std::f32::consts::PI,
                    self.camera.fov + std::f32::consts::FRAC_PI_8 / 2.0,
                )
            }

            Key1 | Numpad1 => self.square.set_selected(VertexSelected::One),
            Key2 | Numpad2 => self.square.set_selected(VertexSelected::Two),
            Key3 | Numpad3 => self.square.set_selected(VertexSelected::Three),
            Key4 | Numpad4 => self.square.set_selected(VertexSelected::Four),

            Left => self.square.displace(Direction::Left, self.displace_amount),
            Right => self.square.displace(Direction::Right, self.displace_amount),
            Up => self.square.displace(Direction::Up, self.displace_amount),
            Down => self.square.displace(Direction::Down, self.displace_amount),

            A => self.camera.displace(Direction::Left, self.displace_amount),
            D => self.camera.displace(Direction::Right, self.displace_amount),
            W => self.camera.displace(Direction::Up, self.displace_amount),
            S => self.camera.displace(Direction::Down, self.displace_amount),

            Q => self.camera.rotate(std::f32::consts::FRAC_PI_8 / 2.0),
            E => self.camera.rotate(-std::f32::consts::FRAC_PI_8 / 2.0),

            _ => {}
        }

        // self.rewrite_texture();

        self.viewport.window.request_redraw();
    }

    // pub fn push_resources(&self, queue: &Queue) -> Result<()> {
    //     self.image.write(queue);

    //     Ok(())
    // }

    pub fn render(&self, device: &Device, queue: &Queue) -> Result<()> {
        let surface_texture = self.viewport.surface.get_current_frame()?.output;
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Main command encoder"),
        });

        queue.write_buffer(&self.radar_buffer, 0, bytemuck::bytes_of(&self.radar));

        let index_buffer = self.square.index_buffer(device);
        let vertex_buffer = self.square.vertex_buffer(device);

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main render pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 5.0 / 256.0,
                            g: 73.0 / 256.0,
                            b: 80.0 / 256.0,
                            a: 1.0,
                        }),
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

    fn load_shader<P: AsRef<Path>>(path: P, device: &Device) -> Result<ShaderModule> {
        let wgsl = std::fs::read_to_string(path)?;
        naga::front::wgsl::parse_str(&wgsl)?;

        let shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Main/radar shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(&wgsl)),
        });

        Ok(shader)
    }

    pub fn reload_shader(&mut self, device: &Device) -> Result<()> {
        let shader = Self::load_shader(&self.shader_path, device)?;

        let bind_group_layout = bind_group_layout(device, &self.radar);
        let pipeline_layout = pipeline_layout(device, &bind_group_layout);
        let format = self.texture_format;

        self.render_pipeline = render_pipeline(device, &shader, &pipeline_layout, &format);

        Ok(())
    }
}
