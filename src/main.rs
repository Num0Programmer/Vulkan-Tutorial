#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()>
{
    pretty_env_logger::init(); // prints logs to console

    // initialize window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768)) // scales window to display
        .build(&event_loop)?;

    // initialize app
    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false; // prevents rendering after app is destroyed
    // rendering loop
    event_loop.run(move |event, _, control_flow|
    {
        *control_flow = ControlFlow::Poll;
        match event
        {
            // render a frame if our vulkan app is not being destroyed
            Event::MainEventsCleared if !destroying
                => unsafe { app.render(&window) }.unwrap(),
            // destroy our vulkan app
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>
            {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            _ => {}
        }
    });
}

/// vulkan application
#[derive(Clone, Debug)]
struct App {}

impl App
{
    /// creates our vulkan app
    unsafe fn create(window: &Window) -> Result<Self>
    {
        Ok(Self {})
    }

    /// renders a frame from our vulkan application
    unsafe fn render(&mut self, window: &Window) -> Result <()>
    {
        Ok(())
    }

    /// destroys our vulkan application
    unsafe fn destroy(&mut self) {}
}

/// the vulkan handles and associated properties used by our vulkan app
#[derive(Clone, Debug, Default)]
struct AppData {}

