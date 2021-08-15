use color_eyre::Result;
use image::GenericImageView;
use std::{fs::File, io::BufReader, num::NonZeroU32};
use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue,
    Sampler, SamplerDescriptor, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsage, TextureView, TextureViewDescriptor,
};

pub struct TextureImage {
    pub texture: Texture,
    pub texture_view: TextureView,
    pub sampler: Sampler,
    pub extent: Extent3d,
    pub data: Vec<u8>,
}

impl TextureImage {
    pub fn new(device: &Device, path: &str) -> Result<Self> {
        let format = image::ImageFormat::from_path(path)?;
        let reader = BufReader::new(File::open(path)?);

        let image = image::load(reader, format)?;
        let data = image.to_rgba8().to_vec();

        let dimensions = image.dimensions();
        let extent = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            texture_view,
            sampler,
            extent,
            data,
        })
    }

    pub fn write(&self, queue: &Queue) {
        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &self.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(self.extent.width * 4),
                rows_per_image: NonZeroU32::new(self.extent.height),
            },
            self.extent,
        )
    }
}
