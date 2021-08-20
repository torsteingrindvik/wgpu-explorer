use color_eyre::Result;
use image::GenericImageView;
use std::{fs::File, io::BufReader, num::NonZeroU32};
use wgpu::{
    Color, Device, Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Texture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

pub struct TextureImage {
    pub texture: Texture,
    pub texture_view: TextureView,
    pub extent: Extent3d,
    pub data: Vec<u8>,
}

/// A texture + image in Rgba8UnormSrgb format.
impl TextureImage {
    pub fn new(
        label: &str,
        device: &Device,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<Self> {
        let data = data.to_owned();
        let extent = Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        Ok(Self {
            texture,
            texture_view,
            extent,
            data,
        })
    }

    pub fn new_from_path(label: &str, device: &Device, path: &str) -> Result<Self> {
        let format = image::ImageFormat::from_path(path)?;
        let reader = BufReader::new(File::open(path)?);

        let image = image::load(reader, format)?;
        let data = image.to_rgba8();

        let dimensions = image.dimensions();

        Self::new(
            label,
            device,
            dimensions.0 as usize,
            dimensions.1 as usize,
            &data,
        )
    }

    pub fn write(&self, queue: &Queue) {
        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
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

    fn size_of_pixel() -> usize {
        4
    }

    fn color_to_rgba(color: &Color) -> (u8, u8, u8, u8) {
        (
            (color.r * 256.0) as u8,
            (color.g * 256.0) as u8,
            (color.b * 256.0) as u8,
            (color.a * 256.0) as u8,
        )
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel_size = Self::size_of_pixel();

        let column_size = pixel_size;
        let row_size = self.extent.width as usize * column_size;

        let pos = (x * column_size) + (y * row_size);

        let (red, green, blue, alpha) = Self::color_to_rgba(&color);
        self.data[pos] = red;
        self.data[pos + 1] = green;
        self.data[pos + 2] = blue;
        self.data[pos + 3] = alpha;
    }
}
