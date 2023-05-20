#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::Version;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

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

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance>
{
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER)
    {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED
    {
        vec![VALIDATION_LAYER.as_ptr()]
    }
    else
    {
        Vec::new()
    };

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // required by Vulkan SDK on macOS since 1.3.216
    let flags =
        if cfg!(target_os = "macos")
            && entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        info!("Enabling extensions for macOS porability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    }
    else
    {
        vk::InstanceCreateFlags::empty()
    };

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    Ok(entry.create_instance(&info, None)?)
}

/// vulkan application
#[derive(Clone, Debug)]
struct App
{
    entry: Entry,
    instance: Instance
}

impl App
{
    /// creates our vulkan app
    unsafe fn create(window: &Window) -> Result<Self>
    {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }

    /// renders a frame from our vulkan application
    unsafe fn render(&mut self, window: &Window) -> Result <()>
    {
        Ok(())
    }

    /// destroys our vulkan application
    unsafe fn destroy(&mut self)
    {
        self.instance.destroy_instance(None);
    }
}

/// the vulkan handles and associated properties used by our vulkan app
#[derive(Clone, Debug, Default)]
struct AppData {}

