use rand;
use rand::Rng;

use glow::*;
use iced_glow::glow;

use crate::buffer_renderer::BufferRenderer;
use crate::buffer_renderer::FramebufferRenderer;
use crate::graphic_config::GraphicConfig;
use crate::texture_util::TextureUtil;
use media_handler::frame::Frame;

use nalgebra_glm::{scale, translate, translation, vec3, TMat4, TVec3};

pub struct GlProgram {
    texture: glow::NativeTexture,
    pub main_renderer: BufferRenderer,
    pub framebuffer_renderer: FramebufferRenderer,
}
impl TextureUtil for GlProgram {}

impl GlProgram {
    pub fn new(gl: &glow::Context, config: &GraphicConfig, win_size: (i32, i32)) -> Self {
        unsafe {
            /*
                Create cudi program
                Create framebuffer program
                Create the main texture
                Create the render scene
            */
            let main_renderer = BufferRenderer::new(
                gl,
                &config.vertex_path,
                &config.fragment_path,
                config.loading_media.ratio,
            );

            let framebuffer_renderer = FramebufferRenderer::new(
                gl,
                &config.fbo_vertex_path,
                &config.fbo_fragment_path,
                win_size,
            );
            let texture = Self::init_texture(gl);
            Self::generate_texture(gl, texture, &config.loading_media);

            gl.use_program(None);
            Self {
                main_renderer,
                framebuffer_renderer,
                texture,
            }
        }
    }

    pub fn draw(&mut self, gl: &glow::Context, media: Option<Frame>, viewport_ratio: f32) {
        let mut rng = rand::thread_rng();
        let cubes_indices: [TVec3<f32>; 1] = [vec3(0.0, 0.0, -3.0)];

        if let Some(m) = media {
            self.main_renderer.scene.ratio = m.ratio;
            self.main_renderer.scene.last_pos = vec3(
                rng.gen_range(-1.2..1.2),
                rng.gen_range(-1.2..1.2),
                rng.gen_range(-1.2..1.2),
            );
            Self::generate_texture(gl, self.texture, &m);
        }

        unsafe {
            //  1. Render in the framebuffer texture
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
            // don't clear the framebuffer GL_COLOR_BUFFER_BIT to keep last buffer data
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.enable(glow::DEPTH_TEST);

            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.use_program(Some(self.main_renderer.program));

            self.main_renderer.scene.update_scene(gl);

            gl.bind_vertex_array(Some(self.main_renderer.vao));
            for (_, position) in cubes_indices.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: TMat4<f32> =
                    translate(&translation(&position), &self.main_renderer.scene.last_pos);
                model = scale(
                    &model,
                    &vec3(
                        self.main_renderer.scene.ratio * 0.5,
                        viewport_ratio * 0.5,
                        1.,
                    ),
                );
                self.main_renderer.scene.update_model(gl, model);
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
            // Unbind everything to clean
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);

            // 2. Bind default framebuffer, draw a plane and show the texture scene
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.disable(glow::DEPTH_TEST);

            gl.use_program(Some(self.framebuffer_renderer.program));
            gl.bind_vertex_array(Some(self.framebuffer_renderer.vao));
            gl.bind_texture(
                glow::TEXTURE_2D,
                Some(self.framebuffer_renderer.color_texture_buffer),
            );
            gl.draw_arrays(glow::TRIANGLES, 0, 6)
        }
    }

    pub fn resize_buffer(
        &mut self,
        gl: &glow::Context,
        win_size: (i32, i32),
        config: &GraphicConfig,
    ) {
        self.cleanup(gl);

        self.main_renderer = BufferRenderer::new(
            gl,
            &config.vertex_path,
            &config.fragment_path,
            config.loading_media.ratio,
        );

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
    }

    pub fn clear(&self, gl: &glow::Context) {
        let [r, g, b, a] = self.framebuffer_renderer.bg_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        self.main_renderer.cleanup(gl);
        self.framebuffer_renderer.cleanup(gl);
    }
}
