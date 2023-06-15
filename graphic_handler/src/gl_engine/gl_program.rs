use std::sync::mpsc::Receiver;

use iced_winit::Color;

use glow::*;
use iced_glow::glow;

use crate::gl_engine::buffer_renderer::BufferRenderer;
use crate::gl_engine::framebuffer_renderer::FramebufferRenderer;
use crate::graphic_config::GraphicConfig;
use media_handler::frame::Frame;

use nalgebra_glm::vec3;
use rand::Rng;

pub struct GlProgram {
    first_render: bool,

    pub main_renderers: Vec<BufferRenderer>,
    pub framebuffer_renderer: FramebufferRenderer,
}

impl GlProgram {
    pub fn new(gl: &glow::Context, config: &GraphicConfig) -> Self {
        unsafe {
            /*
                -> The first render is trigger by `resize_buffer` due to iced
                -> The real first init will be there.

                Create dummy cudi program
                Create dummy framebuffer program
                Create the main texture
            */
            let main_renderers = vec![];
            let framebuffer_renderer = FramebufferRenderer::new(
                gl,
                &config.fbo_vertex_path,
                &config.fbo_fragment_path,
                (1, 1),
            );

            gl.use_program(None);
            Self {
                first_render: true,
                main_renderers,
                framebuffer_renderer,
            }
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        rx: &Receiver<Frame>,
        need_refresh: bool,
        ux_data: Color,
    ) {
        // let mut rng = rand::thread_rng();
        for r in &mut self.main_renderers {
            // r.update_scene_data(vec3(rng.gen_range(-1.2..1.2), rng.gen_range(-1.2..1.2), 1.));
            /* optionnal | need to move */
            unsafe {
                gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
            }
            r.draw(gl, rx, need_refresh, ux_data);
        }
        self.framebuffer_renderer.draw(gl);
    }

    pub fn resize_buffer(
        &mut self,
        gl: &glow::Context,
        win_size: (i32, i32),
        viewport_ratio: f32,
        config: &GraphicConfig,
    ) {
        self.cleanup(gl);
        self.main_renderers = (0..config.renderer_size)
            .map(|i| {
                BufferRenderer::new(
                    gl,
                    i,
                    &config,
                    viewport_ratio,
                    // allow the first render and lock it || find a way to change it for cudi renderer
                    false,
                )
            })
            .collect();

        self.framebuffer_renderer = FramebufferRenderer::new(
            gl,
            &config.fbo_vertex_path,
            &config.fbo_fragment_path,
            win_size,
        );

        unsafe {
            // clear framebuffer that will be display
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
        }
        self.clear(gl);
        self.first_render = false;
    }

    pub fn clear(&self, gl: &glow::Context) {
        let [r, g, b, a] = self.framebuffer_renderer.bg_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        for r in &self.main_renderers {
            r.cleanup(gl)
        }
        self.framebuffer_renderer.cleanup(gl);
    }
}
