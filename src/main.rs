use color_eyre::{eyre::ContextCompat, Result};
use log::debug;
use viewport::Viewport;
use wgpu::*;
use window_extra::WindowExtra;
use window_main::WindowMain;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod window_extra;
mod window_main;

mod camera;
mod misc;
mod square;
mod texture_image;
mod vec;
mod vertex;
mod viewport;

async fn run() -> Result<()> {
    let instance = Instance::new(BackendBit::PRIMARY);

    let event_loop = EventLoop::new();

    let window_main = Window::new(&event_loop)?;
    window_main.set_inner_size(PhysicalSize::new(800, 800));
    window_main.set_outer_position(PhysicalPosition::new(0.0, 300.0));

    let window_extra = Window::new(&event_loop)?;
    window_extra.set_inner_size(PhysicalSize::new(1600, 800));
    window_extra.set_outer_position(PhysicalPosition::new(800.0, 300.0));

    let surface = unsafe { instance.create_surface(&window_main) };
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .wrap_err("No adapter")?;

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                ..Default::default()
            },
            None,
        )
        .await?;

    let texture_format = adapter
        .get_swap_chain_preferred_format(&surface)
        .wrap_err("No preferred format for swap chain")?;

    let mut main = WindowMain::new(
        Viewport::new(window_main, &instance, &adapter, &device)?,
        &device,
        // &queue,
        &texture_format,
    )?;

    let mut extra = WindowExtra::new(
        Viewport::new(window_extra, &instance, &adapter, &device)?,
        &device,
        &queue,
        &texture_format,
    )?;

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
                ..
            } => {
                debug!("Resize: {:?}, id: {:?}", size, window_id);

                if window_id == main.viewport.window.id() {
                    main.viewport.resize(&device, size);
                } else if window_id == extra.viewport.window.id() {
                    extra.viewport.resize(&device, size);
                } else {
                    panic!("OTHER WINDOW???");
                }
            }

            Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                debug!("Pressed: {:?}, id: {:?}", key, window_id);

                if window_id == main.viewport.window.id() {
                    main.handle_key(key);
                } else if window_id == extra.viewport.window.id() {
                    extra.handle_key(key);
                } else {
                    panic!("OTHER WINDOW???");
                }
            }

            Event::RedrawRequested(window_id) => {
                debug!("Redraw on id {:?}", window_id);
                if window_id == main.viewport.window.id() {
                    main.render(&device, &queue).expect("Render main gone bad");
                } else if window_id == extra.viewport.window.id() {
                    extra
                        .render(&device, &queue)
                        .expect("Render extra gone bad");
                } else {
                    panic!("OTHER WINDOW???");
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            _ => {}
        }
    });
}

fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn"));

    pollster::block_on(run())
}
