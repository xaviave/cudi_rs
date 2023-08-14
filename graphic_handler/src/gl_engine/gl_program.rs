use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use glow::*;
use iced_glow::glow;
use iced_winit::winit::event::VirtualKeyCode;
use nalgebra_glm::Vec3;

use crate::controls::Controls;
use crate::gl_engine::framebuffer_renderer::FramebufferRenderer;
use crate::gl_engine::scene::Scene;
use crate::graphic_config::GraphicConfig;
use media_handler::frame::Frame;

pub struct GlProgram {
    first_render: bool,

    pub main_scenes: Vec<Scene>,
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

            let main_scenes = (0..config.renderer_size)
                .map(|i| {
                    Scene::new(
                        gl,
                        i,
                        &config,
                        (config.width as f64, config.height as f64),
                        // allow the first render and lock it || find a way to change it for cudi renderer
                        false,
                        ((config.height / 2) as f64, (config.width / 2) as f64),
                    )
                })
                .collect();
            let framebuffer_renderer = FramebufferRenderer::new(
                gl,
                &config.fbo_vertex_path,
                &config.fbo_fragment_path,
                (1, 1),
            );

            gl.use_program(None);
            Self {
                first_render: true,
                main_scenes,
                framebuffer_renderer,
            }
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        rx: &Receiver<Frame>,
        need_refresh: bool,
        ux_data: &Controls,
        keyboard_data: &Vec<VirtualKeyCode>,
        mouse_position: (f64, f64),
    ) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
            gl.enable(glow::DEPTH_TEST);
            let mut mask = glow::DEPTH_BUFFER_BIT;
            // if !self.update_media {
            if need_refresh {
                mask |= glow::COLOR_BUFFER_BIT;
                gl.clear_color(0., 0., 0., 1.);
            }
            gl.clear(mask);
        }
        // let mut rng = rand::thread_rng();
        for s in &mut self.main_scenes {
            // r.update_scene_data(vec3(rng.gen_range(-1.2..1.2), rng.gen_range(-1.2..1.2), 1.));
            /* optionnal | need to move */
            s.draw(gl, rx, need_refresh, ux_data, keyboard_data, mouse_position);
        }
        self.framebuffer_renderer.draw(gl);
    }

    pub fn update_scenes_projection(&mut self, viewport_size: (f64, f64), direction: f32) {
        for s in &mut self.main_scenes {
            let mut fov = s.fov + direction;

            if fov < 1.0 {
                fov = 1.0;
            } else if fov > 90.0 {
                fov = 90.0;
            }
            s.update_projection(viewport_size, fov);
        }
    }

    pub fn resize_buffer(
        &mut self,
        gl: &glow::Context,
        win_size: (i32, i32),
        viewport_ratio: f32,
        config: &GraphicConfig,
    ) {
        self.cleanup(gl);
        let viewport_size = (win_size.0 as f64, win_size.1 as f64);
        for s in &mut self.main_scenes {
            s.init_gl_component(gl, config, viewport_size);
        }

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
    pub fn cleanup(&mut self, gl: &glow::Context) {
        for s in &mut self.main_scenes {
            s.cleanup(gl)
        }
        self.framebuffer_renderer.cleanup(gl);
    }
}
