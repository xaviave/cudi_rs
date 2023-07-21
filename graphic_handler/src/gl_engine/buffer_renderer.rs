use std::path::PathBuf;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;
use nalgebra_glm::Vec3;

use crate::gl_engine::buffer_util::BufferUtil;
use crate::scene::Scene;
use nalgebra_glm::{scale, translate, translation, vec3, TMat4, TVec3};

pub struct BufferRenderer {
    pub vao: glow::VertexArray,
    pub vbo: glow::NativeBuffer,
    pub program: glow::Program,

    pub scene: Scene,
    pub bg_color: Color,
    pub update_media: bool,
}

impl BufferUtil for BufferRenderer {}

impl BufferRenderer {
    pub fn new(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
        ratio: f32,
        update_media: bool,
    ) -> Self {
        /*
            Create graphic program
            Create the render scene
        */
        let (program, vao, vbo) = Self::init_program_buffer(
            gl,
            vertex_path,
            fragment_path,
            &[3, 2],
            &Self::get_vertex_array(),
        );

        let mut scene = Scene::new(gl, &program);
        scene.ratio = ratio;

        Self {
            vao,
            vbo,
            program,
            scene,
            bg_color: Color::new(0., 0., 0., 1.),
            update_media,
        }
    }

    pub fn update_scene_data(&mut self, ratio: f32, last_pos: Vec3) {
        self.scene.ratio = ratio;
        self.scene.last_pos = last_pos;
    }

    pub fn draw(&mut self, gl: &glow::Context, texture: glow::NativeTexture, viewport_ratio: f32) {
        let cubes_indices: [TVec3<f32>; 1] = [vec3(0.0, 0.0, -3.0)];

        if !self.update_media {
            return;
        }
        unsafe {
            //  1. Render in the framebuffer texture
            // don't clear the framebuffer GL_COLOR_BUFFER_BIT to keep last buffer data
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.enable(glow::DEPTH_TEST);

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.use_program(Some(self.program));

            self.scene.update_scene(gl);

            gl.bind_vertex_array(Some(self.vao));
            for (_, position) in cubes_indices.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: TMat4<f32> =
                    translate(&translation(&position), &self.scene.last_pos);
                model = scale(
                    &model,
                    &vec3(self.scene.ratio * 0.1, viewport_ratio * 0.1, 1.),
                );
                self.scene.update_model(gl, model);
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
            // Unbind everything to clean
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }
}
