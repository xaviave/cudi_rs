mod controls;
mod gl_program;
pub mod graphic_config;

use std::time::{Duration, Instant};

use controls::Controls;
use gl_program::GlProgram;
use graphic_config::GraphicConfig;

use glow::*;
use glutin::dpi::PhysicalPosition;
use glutin::event::{Event, ModifiersState, WindowEvent};
use glutin::event_loop::ControlFlow;
use iced_glow::glow;
use iced_glow::{Backend, Renderer, Settings, Viewport};
use iced_glutin::conversion;
use iced_glutin::glutin::{self, ContextWrapper};
use iced_glutin::program::State;
use iced_glutin::renderer;
use iced_glutin::{program, Clipboard, Color, Debug, Size};
use media_handler::media_handler::MediaHandler;

pub struct GraphicContext {
    config: GraphicConfig,

    gl: Context,
    windowed_context: ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    event_loop: glutin::event_loop::EventLoop<()>,

    viewport: Viewport,
    program: GlProgram,
    renderer: Renderer,

    state: State<Controls>,
    cursor_position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
    clipboard: Clipboard,
    resized: bool,
    debug: Debug,
}

impl GraphicContext {
    pub fn new(config: GraphicConfig) -> Self {
        env_logger::init();

        let event_loop = glutin::event_loop::EventLoop::new();

        let (gl, windowed_context) = {
            let wb = glutin::window::WindowBuilder::new()
                .with_title(&config.app_name)
                .with_inner_size(glutin::dpi::LogicalSize::new(config.width, config.height));

            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(wb, &event_loop)
                .unwrap();

            unsafe {
                let windowed_context = windowed_context.make_current().unwrap();

                let gl = glow::Context::from_loader_function(|s| {
                    windowed_context.get_proc_address(s) as *const _
                });

                // Enable auto-conversion from/to sRGB
                gl.enable(glow::FRAMEBUFFER_SRGB);

                // Enable alpha blending
                gl.enable(glow::BLEND);
                gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

                // // Disable multisampling by default
                // gl.disable(glow::MULTISAMPLE);

                (gl, windowed_context)
            }
        };

        let physical_size = windowed_context.window().inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            windowed_context.window().scale_factor(),
        );

        let mut debug = Debug::new();
        let controls = Controls::new();
        let modifiers = ModifiersState::default();
        let program = GlProgram::new(
            &gl,
            &config.vertex_path,
            &config.fragment_path,
            &config.loading_media,
        );
        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let clipboard = Clipboard::connect(windowed_context.window());
        let mut renderer = Renderer::new(Backend::new(&gl, Settings::default()));
        let state =
            program::State::new(controls, viewport.logical_size(), &mut renderer, &mut debug);

        Self {
            config,
            gl,
            windowed_context,
            event_loop,
            viewport,
            program,
            renderer,
            state,
            cursor_position,
            modifiers,
            clipboard,
            resized: false,
            debug,
        }
    }

    pub fn launch_graphic(mut self, mut media_handler: MediaHandler) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            self.cursor_position = position;
                        }
                        WindowEvent::ModifiersChanged(new_modifiers) => {
                            self.modifiers = new_modifiers;
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.viewport = Viewport::with_physical_size(
                                Size::new(physical_size.width, physical_size.height),
                                self.windowed_context.window().scale_factor(),
                            );

                            self.resized = true;
                        }
                        WindowEvent::CloseRequested => unsafe {
                            self.program.cleanup(&self.gl);
                            *control_flow = ControlFlow::Exit
                        },
                        _ => (),
                    }

                    // Map window event to iced event
                    if let Some(event) = iced_winit::conversion::window_event(
                        &event,
                        self.windowed_context.window().scale_factor(),
                        self.modifiers,
                    ) {
                        self.state.queue_event(event);
                    }
                }
                Event::MainEventsCleared => {
                    // If there are events pending
                    if !self.state.is_queue_empty() {
                        // We update iced
                        let _ = self.state.update(
                            self.viewport.logical_size(),
                            conversion::cursor_position(
                                self.cursor_position,
                                self.viewport.scale_factor(),
                            ),
                            &mut self.renderer,
                            &iced_glow::Theme::Dark,
                            &renderer::Style {
                                text_color: Color::WHITE,
                            },
                            &mut self.clipboard,
                            &mut self.debug,
                        );

                        // and request a redraw
                        self.windowed_context.window().request_redraw();
                    }
                }
                Event::RedrawRequested(_) => {
                    if self.resized {
                        let size = self.windowed_context.window().inner_size();

                        unsafe {
                            self.gl
                                .viewport(0, 0, size.width as i32, size.height as i32);
                        }
                        self.resized = false;
                    }

                    let control = self.state.program();
                    unsafe {
                        // We clear the frame
                        self.program.clear(&self.gl, control.background_color);
                        // Draw the scene
                        self.program
                            .draw(&self.gl, Some(&media_handler.get_next_media()));
                    }
                    // And then iced on top
                    self.renderer.with_primitives(|backend, primitive| {
                        backend.present(&self.gl, primitive, &self.viewport, &self.debug.overlay());
                    });

                    // Update the mouse cursor
                    self.windowed_context.window().set_cursor_icon(
                        iced_winit::conversion::mouse_interaction(self.state.mouse_interaction()),
                    );
                    self.windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
