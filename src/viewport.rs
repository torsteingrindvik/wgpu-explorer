use color_eyre::{eyre::ContextCompat, Result};
use wgpu::{
    Adapter, Device, Instance, PresentMode, Surface, SwapChain, SwapChainDescriptor, TextureUsage,
};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Viewport {
    pub window: Window,
    pub surface: Surface,
    pub swap_chain_descriptor: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

impl Viewport {
    pub fn new(
        window: Window,
        instance: &Instance,
        adapter: &Adapter,
        device: &Device,
    ) -> Result<Self> {
        let surface = unsafe { instance.create_surface(&window) };

        let size = window.inner_size();
        let swap_chain_descriptor = SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT,
            format: adapter
                .get_swap_chain_preferred_format(&surface)
                .wrap_err("No preferred format")?,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        Ok(Self {
            window,
            surface,
            swap_chain_descriptor,
            swap_chain,
        })
    }

    pub fn resize(&mut self, device: &Device, size: PhysicalSize<u32>) {
        self.swap_chain_descriptor.width = size.width;
        self.swap_chain_descriptor.height = size.height;

        self.swap_chain = device.create_swap_chain(&self.surface, &self.swap_chain_descriptor);
    }
}
