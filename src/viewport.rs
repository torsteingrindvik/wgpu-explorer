use color_eyre::{eyre::ContextCompat, Result};
use wgpu::{Adapter, Device, Instance, PresentMode, Surface, SurfaceConfiguration, TextureUsages};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Viewport {
    pub window: Window,
    pub surface: Surface,
}

impl Viewport {
    fn configure_surface(
        &mut self,
        device: &Device,
        adapter: &Adapter,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self
                .surface
                .get_preferred_format(adapter)
                .wrap_err("No preferred format")?,
            width,
            height,
            present_mode: PresentMode::Fifo,
        };
        self.surface.configure(device, &config);

        Ok(())
    }

    pub fn new(
        window: Window,
        instance: &Instance,
        adapter: &Adapter,
        device: &Device,
    ) -> Result<Self> {
        let surface = unsafe { instance.create_surface(&window) };

        let size = window.inner_size();
        let mut new_self = Self { window, surface };
        new_self.configure_surface(device, adapter, size.width, size.height)?;

        Ok(new_self)
    }

    pub fn resize(&mut self, adapter: &Adapter, device: &Device, size: PhysicalSize<u32>) {
        self.configure_surface(device, adapter, size.width, size.height)
            .unwrap();
    }
}
