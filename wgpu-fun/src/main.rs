// https://youtube.com/playlist?list=PLn3eTxaOtL2PNbW4ou-APMV9W9m6nppYl

use glfw::{Action, ClientApiHint, Key, Window, WindowHint, fail_on_errors};
mod renderer_backend;
use renderer_backend::pipeline_builder::PipelineBuilder;

use crate::renderer_backend::pipeline_builder;

struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> {
    async fn new(window: &'a mut Window) -> Self {
        let size = window.get_framebuffer_size();

        // Fix 1: Instance::new() takes by value (no &), not by reference
        // Fix 2: InstanceDescriptor::default() removed — construct all fields explicitly
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::from_build_config(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None,
        });

        let surface = instance.create_surface(window.render_context()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Fix 3: request_device() now takes only 1 argument (no trace path argument)
        // Fix 4: DeviceDescriptor gained experimental_features, memory_hints, trace
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        }
    }

    fn resize(&mut self, new_size: (i32, i32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update_surface(&mut self) {
        self.surface = self
            .instance
            .create_surface(self.window.render_context())
            .unwrap();
        self.surface.configure(&self.device, &self.config);
    }

    // Fix 5: SurfaceError is gone — render() no longer returns a Result
    fn render(&mut self) {
        // Fix 6: get_current_texture() now returns CurrentSurfaceTexture enum, not Result
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(drawable) => {
                let image_view = drawable
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut command_encoder =
                    self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                {
                    let mut render_pass =
                        command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &image_view,
                                resolve_target: None,
                                depth_slice: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.75,
                                        g: 0.5,
                                        b: 0.25,
                                        a: 1.0,
                                    }),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                            multiview_mask: None,
                        });
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.draw(0..3, 0..1);
                }

                self.queue.submit(std::iter::once(command_encoder.finish()));
                drawable.present();
            }

            // Surface is out of date or lost — caller should reconfigure
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                let size = self.size;
                self.update_surface();
                self.resize(size);
            }

            // Suboptimal but still renderable — reconfigure after presenting
            wgpu::CurrentSurfaceTexture::Suboptimal(drawable) => {
                drawable.present();
                let size = self.size;
                self.update_surface();
                self.resize(size);
            }

            // Skip this frame silently
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {}

            wgpu::CurrentSurfaceTexture::Validation => {
                eprintln!("wgpu validation error during surface acquisition");
            }
        }
    }
}

async fn run() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(800, 600, "It's WGPU time.", glfw::WindowMode::Windowed)
        .unwrap();

    let mut state = State::new(&mut window).await;

    state.window.set_framebuffer_size_polling(true);
    state.window.set_key_polling(true);
    state.window.set_mouse_button_polling(true);
    state.window.set_pos_polling(true);

    while !state.window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    state.window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
                    state.window.set_should_close(true);
                }
                glfw::WindowEvent::Pos(..) => {
                    state.update_surface();
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    state.resize((width, height));
                }
                _ => {}
            }
        }

        state.render();
    }
}

// Chapter 3: Shaders
// 1. write shader
// 2. program that reads shader

fn main() {
    pollster::block_on(run());
}
