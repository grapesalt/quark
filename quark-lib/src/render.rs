use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use vello::{
    util::{RenderContext, RenderSurface},
    wgpu, AaConfig, RenderParams, Renderer, RendererOptions,
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::Scene;

struct ActiveState<'a> {
    window: Arc<Window>,
    surface: RenderSurface<'a>,
    renderer: Renderer,
    vscene: vello::Scene,
}

struct PreviewApp<'a> {
    scene: Scene,
    context: RenderContext,
    start: Instant,
    state: Option<ActiveState<'a>>,
    paused: bool,
    pause_time: f64,
}

impl<'s> ApplicationHandler for PreviewApp<'s> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.init(event_loop).unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.handle_event(event_loop, event).unwrap();
    }
}

impl<'s> PreviewApp<'s> {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Quark Preview")
                        .with_inner_size(winit::dpi::LogicalSize::new(
                            self.scene.width,
                            self.scene.height,
                        )),
                )
                .context("Failed to create a window.")?,
        );

        let size = window.inner_size();

        let surface = pollster::block_on(self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        ))
        .context("Failed to create a render surface.")?;

        let device = &self.context.devices[surface.dev_id].device;

        let renderer = Renderer::new(
            device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: std::num::NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )
        .map_err(|e| anyhow::anyhow!("{e}"))
        .context("Failed to create the vello renderer.")?;

        self.state = Some(ActiveState {
            window,
            surface,
            renderer,
            vscene: vello::Scene::new(),
        });

        Ok(())
    }

    fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) -> Result<()> {
        let state = match &mut self.state {
            Some(s) => s,
            None => return Ok(()),
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::Escape) | PhysicalKey::Code(KeyCode::KeyQ) => {
                            event_loop.exit();
                        }
                        PhysicalKey::Code(KeyCode::Space) => {
                            if self.paused {
                                self.start =
                                    Instant::now() - Duration::from_secs_f64(self.pause_time);
                                self.paused = false;
                            } else {
                                self.pause_time = self.start.elapsed().as_secs_f64();
                                self.paused = true;
                            }
                        }
                        PhysicalKey::Code(KeyCode::Digit0) => {
                            self.start = Instant::now();
                            self.paused = false;
                        }
                        PhysicalKey::Code(KeyCode::ArrowLeft) => {
                            let rewound = (self.start.elapsed().as_secs_f64() - 2.0).max(0.0);

                            self.start = Instant::now() - Duration::from_secs_f64(rewound);
                            self.paused = false;
                        }
                        PhysicalKey::Code(KeyCode::ArrowRight) => {
                            let forward = self.start.elapsed().as_secs_f64() + 2.0;

                            self.start = Instant::now() - Duration::from_secs_f64(forward);
                            self.paused = false;
                        }
                        _ => {}
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                let t = if self.paused {
                    self.pause_time
                } else {
                    self.start.elapsed().as_secs_f64()
                };

                self.scene.draw_at(t, &mut state.vscene);

                let bg = self.scene.background.to_vello_color();
                let device_handle = &self.context.devices[state.surface.dev_id];

                state
                    .renderer
                    .render_to_texture(
                        &device_handle.device,
                        &device_handle.queue,
                        &state.vscene,
                        &state.surface.target_view,
                        &RenderParams {
                            base_color: bg,
                            width: state.surface.config.width,
                            height: state.surface.config.height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .map_err(|e| anyhow::anyhow!("{e}"))
                    .context("Failed to render frame.")?;

                let surface_texture = state
                    .surface
                    .surface
                    .get_current_texture()
                    .context("Failed to get the surface texture.")?;

                let surface_view = surface_texture.texture.create_view(&Default::default());

                let mut encoder = device_handle
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                state.surface.blitter.copy(
                    &device_handle.device,
                    &mut encoder,
                    &state.surface.target_view,
                    &surface_view,
                );

                device_handle.queue.submit([encoder.finish()]);
                surface_texture.present();

                state.window.request_redraw();
            }

            _ => {}
        }

        Ok(())
    }
}

pub(crate) fn run_preview(scene: Scene) -> Result<()> {
    let event_loop = EventLoop::new().context("Failed to create the event loop.")?;

    let mut app = PreviewApp {
        scene,
        context: RenderContext::new(),
        start: Instant::now(),
        state: None,
        paused: false,
        pause_time: 0.0,
    };

    event_loop.run_app(&mut app).context("Event loop failed.")?;

    Ok(())
}
