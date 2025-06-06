use pollster::FutureExt;
use winit::window::Window;

use std::sync::Arc;


pub(crate) struct Wgpu {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,
}
impl Wgpu {
    // Создание холста
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();


        // Выбор бэкенда
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY, // По приоритету (Vulkan > metal > dx12 > webwgpu)
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL, // Для браузера (webgl)
            ..Default::default()
        });


        let surface = instance.create_surface(window.clone()).unwrap();

        // Связь с видеокартой
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance, // Выделение самой мощной видеокарты
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).block_on().unwrap();


        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default() // Лимитер для большинства устройств
                },
                label: None,
                memory_hints: Default::default(), // По умолчанию приоритет на производительность
                trace: wgpu::Trace::Off,
            },
        ).block_on().unwrap();


        let surface_caps = surface.get_capabilities(&adapter);

        // Установка sRGB для сохранения текстур в GPU
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        // Конфигурация холста
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width, // Ширина холста
            height: size.height, // Высота холста
            present_mode: surface_caps.present_modes[0], // VSync (вроде)
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }


    // Каждый кадр
    pub fn render(&mut self) {
        // Холст
        let output = self.surface.get_current_texture().unwrap();
        // Обзор
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Буфер для комманд GPU
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })], // Фоновый цвет
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }


    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}