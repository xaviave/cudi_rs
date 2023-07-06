use std::sync::mpsc::Receiver;

use glow::*;
use iced_glow::glow;
use media_handler::frame::Frame;

use crate::gl_engine::material::CMaterial;

use super::{buffer_renderer::BufferRenderer, texture_util::TextureUtil};

pub struct Model {
    pub raw_vertex_buffer: Vec<f32>,
    pub raw_indices_buffer: Vec<i32>,

    pub material_data: CMaterial,
    pub update_media: bool,

    pub gl_buffer: BufferRenderer,
    pub texture: glow::NativeTexture,
}

impl TextureUtil for Model {}

impl Model {
    pub fn new(
        gl: &Context,
        raw_vertex_buffer: Vec<f32>,
        raw_indices_buffer: Vec<i32>,
        material_data: CMaterial,
        update_media: bool,
    ) -> Self {
        Self {
            raw_vertex_buffer: raw_vertex_buffer.clone(),
            raw_indices_buffer: raw_indices_buffer.clone(),
            material_data,
            update_media,
            gl_buffer: BufferRenderer::new(gl, &raw_vertex_buffer, &raw_indices_buffer),
            texture: Self::init_texture(gl),
        }
    }

    // fn set_media_texture(&mut self, gl: &glow::Context, rx: &Receiver<Frame>, need_refresh: bool) {
    //     if need_refresh && self.update_media {
    //         let new_media = match rx.recv() {
    //             Ok(f) => Some(f),
    //             Err(_) => None,
    //         };
    //         if let Some(m) = new_media {
    //             // self.model.texture_ratio = m.ratio;
    //             Self::generate_texture(gl, self.texture, &m);
    //         }
    //         unsafe {
    //             gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
    //         }
    //     }
    // }

    pub fn draw(&mut self, gl: &glow::Context, program: glow::NativeProgram, rx: &Receiver<Frame>) {
        // self.set_media_texture(gl, rx, need_refresh);
        self.material_data.update_material(gl, program);

        unsafe {
            // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
            gl.bind_vertex_array(Some(self.gl_buffer.vao));
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.draw_elements(
                glow::TRIANGLES,
                self.raw_indices_buffer.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );

            // Unbind everything to clean
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        // let mut child = Command::new("sleep").arg("0.01").spawn().unwrap();
        // let _result = child.wait().unwrap();
    }

    pub fn init_gl_component(&mut self, gl: &glow::Context) {
        self.gl_buffer = BufferRenderer::new(gl, &self.raw_vertex_buffer, &self.raw_indices_buffer);
        self.texture = Model::init_texture(gl);
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        self.gl_buffer.cleanup(gl);

        unsafe {
            gl.delete_texture(self.texture);
        }
    }
}
