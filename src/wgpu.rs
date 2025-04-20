use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration};

use crate::window::Window;

pub struct WGPU<'a> {
    instance: Instance,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl <'a> WGPU<'a> {
    pub async unsafe fn new(window: &Window) -> Self {
        unsafe {
            let descriptor = wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(), ..Default::default()
            };

            let instance = Instance::new(&descriptor);
            
            let target = unsafe {
                wgpu::SurfaceTargetUnsafe::from_window(&window)
            }.expect("Failed to create surface target.");

            let surface = instance
                .create_surface_unsafe(target)
                .expect("Failed to create surface.");

            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ).await.unwrap();

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                },
            ).await.unwrap();

            let surface_caps = surface.get_capabilities(&adapter);

            let surface_format = surface_caps.formats.iter()
                .find(|f| f.is_srgb())
                .copied()
                .unwrap_or(surface_caps.formats[0]);

            let (width, height) = window.get_size();

            let config = SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: width as u32,
                height: height as u32,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            WGPU {
                instance,
                surface,
                device,
                queue,
                config
            }
        }
    }
}