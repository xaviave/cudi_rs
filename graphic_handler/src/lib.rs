mod controls;
mod gl_program;
pub mod graphic_config;
mod scene;

use controls::Controls;
use gl_program::GlProgram;
use graphic_config::GraphicConfig;
use media_handler::media_handler::MediaHandler;

use std::time::Instant;

use glow::*;
use iced_glow::*;

use iced_glutin::glutin;
use iced_glutin::*;

pub struct GraphicContext {
    config: GraphicConfig,

    gl: Context,
    windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    event_loop: glutin::event_loop::EventLoop<()>,

    viewport: Viewport,
    program: GlProgram,
    renderer: iced_glow::Renderer,

    state: program::State<Controls>,
    cursor_position: glutin::dpi::PhysicalPosition<f64>,
    modifiers: glutin::event::ModifiersState,
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

                // Disable multisampling by default
                gl.disable(glow::MULTISAMPLE);

                gl.enable(glow::DEPTH_TEST);
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
        let modifiers = glutin::event::ModifiersState::default();
        let program = GlProgram::new(
            &gl,
            &config.vertex_path,
            &config.fragment_path,
            &config.loading_media,
            (physical_size.width as i32, physical_size.height as i32),
        );
        let cursor_position = glutin::dpi::PhysicalPosition::new(-1.0, -1.0);
        let clipboard = Clipboard::connect(windowed_context.window());
        let mut renderer =
            iced_glow::Renderer::new(Backend::new(&gl, iced_glow::Settings::default()));
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
        let mut need_clear: u8 = 0;
        let mut next_media = false;
        let mut current_time = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Poll;

            match event {
                glutin::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::event::WindowEvent::CursorMoved { position, .. } => {
                            self.cursor_position = position;
                        }
                        glutin::event::WindowEvent::ModifiersChanged(new_modifiers) => {
                            self.modifiers = new_modifiers;
                        }
                        glutin::event::WindowEvent::Resized(physical_size) => {
                            self.viewport = Viewport::with_physical_size(
                                Size::new(physical_size.width, physical_size.height),
                                self.windowed_context.window().scale_factor(),
                            );
                            need_clear = 2;
                            self.resized = true;
                        }
                        glutin::event::WindowEvent::CloseRequested => {
                            self.program.cleanup(&self.gl);
                            *control_flow = glutin::event_loop::ControlFlow::Exit
                        }
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
                glutin::event::Event::MainEventsCleared => {
                    // If there are events pending
                    if !self.state.is_queue_empty() {
                        self.state.update(
                            self.viewport.logical_size(),
                            conversion::cursor_position(
                                self.cursor_position,
                                self.viewport.scale_factor(),
                            ),
                            &mut self.renderer,
                            &Theme::Dark,
                            &renderer::Style {
                                text_color: Color::WHITE,
                            },
                            &mut self.clipboard,
                            &mut self.debug,
                        );
                    }

                    if current_time.elapsed().as_millis() > self.config.fps {
                        current_time = Instant::now();
                        next_media = true;
                    }
                    self.windowed_context.window().request_redraw();
                }
                glutin::event::Event::RedrawRequested(_) => {
                    if self.resized {
                        let size = self.windowed_context.window().inner_size();

                        unsafe {
                            self.gl
                                .viewport(0, 0, size.width as i32, size.height as i32);
                        }
                        self.resized = false;
                        need_clear = 2;
                    }

                    // double buffer need 2 clear
                    if need_clear > 0 {
                        self.program
                            .clear(&self.gl, self.state.program().background_color);
                    }
                    self.program.draw(
                        &self.gl,
                        if next_media {
                            Some(media_handler.get_next_media())
                        } else {
                            None
                        },
                    );
                    next_media = false;

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
