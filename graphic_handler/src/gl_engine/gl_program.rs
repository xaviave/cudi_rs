use std::sync::mpsc::Receiver;

use rand;
use rand::Rng;

use glow::*;
use iced_glow::glow;

use crate::gl_engine::buffer_renderer::BufferRenderer;
use crate::gl_engine::framebuffer_renderer::FramebufferRenderer;
use crate::gl_engine::texture_util::TextureUtil;
use crate::graphic_config::GraphicConfig;
use media_handler::frame::Frame;

use nalgebra_glm::vec3;

pub struct GlProgram {
    first_render: bool,
    texture: glow::NativeTexture,
    pub main_renderers: Vec<BufferRenderer>,
    pub framebuffer_renderer: FramebufferRenderer,
}
impl TextureUtil for GlProgram {}

impl GlProgram {
    pub fn new(gl: &glow::Context, config: &GraphicConfig, win_size: (i32, i32)) -> Self {
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
            let texture = Self::init_texture(gl);

            gl.use_program(None);
            Self {
                first_render: false,
                main_renderers,
                framebuffer_renderer,
                texture,
            }
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        rx: &Receiver<Frame>,
        next_media: bool,
        viewport_ratio: f32,
    ) {
        let mut rng = rand::thread_rng();

        let mut ratio = 0.;
        for r in &mut self.main_renderers {
            // for each renderers, ask a different media
            let media = if next_media && r.update_media {
                match rx.recv() {
                    Ok(f) => Some(f),
                    Err(_) => None,
                }
            } else {
                None
            };
            if let Some(m) = media {
                ratio = m.ratio;
                Self::generate_texture(gl, self.texture, &m);
            }

            r.update_scene_data(
                ratio,
                vec3(rng.gen_range(-1.2..1.2), rng.gen_range(-1.2..1.2), 1.),
            );
            unsafe {
                gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
            }
            r.draw(gl, self.texture, viewport_ratio);
        }
        self.framebuffer_renderer.draw(gl);
    }

    pub fn resize_buffer(
        &mut self,
        gl: &glow::Context,
        win_size: (i32, i32),
        config: &GraphicConfig,
    ) {
        self.cleanup(gl);
        // doesn't work for static media if resize || it will be reset and not re render with the unique texture
        self.main_renderers = (0..config.renderer_size)
            .map(|_| {
                BufferRenderer::new(
                    gl,
                    &config.vertex_path,
                    &config.fragment_path,
                    config.loading_media.ratio,
                    // allow the first render and lock it 
                    true || self.first_render,
                )
            })
            .collect();

        self.framebuffer_renderer = FramebufferRenderer::new(
            gl,
            &config.fbo_vertex_path,
            &config.fbo_fragment_path,
            win_size,
        );
        self.texture = Self::init_texture(gl);

        // clear framebuffer that will be display
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
        }
        self.clear(gl);
        self.first_render = true;
    }

    pub fn clear(&self, gl: &glow::Context) {
        let [r, g, b, a] = self.framebuffer_renderer.bg_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.texture);
        }

        for r in &self.main_renderers {
            r.cleanup(gl)
        }
        self.framebuffer_renderer.cleanup(gl);
    }
}
