use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc::Receiver,
};

use glow::*;
use iced_glow::glow;
use media_handler::frame::Frame;
use obj::raw::material::{Material, MtlTextureMap};

use crate::gl_engine::material::CMaterial;

use super::{buffer_renderer::BufferRenderer, gl_error::gl_error, texture_util::TextureUtil};

pub struct Model {
    pub raw_vertex_buffer: Vec<f32>,
    pub raw_indices_buffer: Vec<i32>,

    pub material_data: CMaterial,
    ambient_map_path: Option<PathBuf>,
    diffuse_map_path: Option<PathBuf>,
    specular_map_path: Option<PathBuf>,
    // normal map
    bump_map_path: Option<PathBuf>,

    pub update_media: bool,
    pub gl_buffer: BufferRenderer,
}

impl TextureUtil for Model {}

impl Model {
    pub fn new(
        gl: &Context,
        program: NativeProgram,
        raw_vertex_buffer: Vec<f32>,
        raw_indices_buffer: Vec<i32>,
        update_media: bool,
        mtl_path: &Path,
        raw_material: &Material,
    ) -> Self {
        let get_path = |file: &Option<MtlTextureMap>| -> Option<PathBuf> {
            match file {
                Some(map) => Some(mtl_path.join(&map.file)),
                _ => None,
            }
        };

        Self {
            raw_vertex_buffer: raw_vertex_buffer.clone(),
            raw_indices_buffer: raw_indices_buffer.clone(),
            material_data: CMaterial::new(gl, program, raw_material),
            ambient_map_path: get_path(&raw_material.ambient_map),
            diffuse_map_path: get_path(&raw_material.diffuse_map),
            specular_map_path: get_path(&raw_material.specular_map),
            bump_map_path: get_path(&raw_material.bump_map),
            update_media,
            gl_buffer: BufferRenderer::new(gl, &raw_vertex_buffer, &raw_indices_buffer),
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

    fn bind_textures(
        &mut self,
        gl: &glow::Context,
        textures: &HashMap<PathBuf, Option<glow::NativeTexture>>,
    ) {
        let activate_texture = |map_path: &Option<PathBuf>, i: u32| match map_path {
            Some(t) => match textures.get(t) {
                Some(review) => unsafe {
                    gl.active_texture(i);
                    gl.bind_texture(glow::TEXTURE_2D, review.clone());
                    gl_error(gl, String::from("Aie"));
                },
                None => (),
            },
            _ => (),
        };

        activate_texture(&self.ambient_map_path, 0);
        activate_texture(&self.diffuse_map_path, 1);
        // activate_texture(&self.specular_map_path, 2);
        // activate_texture(&self.bump_map_path, 3);
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        program: glow::NativeProgram,
        rx: &Receiver<Frame>,
        textures: &HashMap<PathBuf, Option<glow::NativeTexture>>,
    ) {
        // self.set_media_texture(gl, rx, need_refresh);
        self.material_data.update_material(gl, program);

        unsafe {
            // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
            gl.bind_vertex_array(Some(self.gl_buffer.vao));
            self.bind_textures(gl, textures);
            gl.draw_elements(
                glow::TRIANGLES,
                self.raw_indices_buffer.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );

            // Unbind everything to clean.
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn init_gl_component(&mut self, gl: &glow::Context) {
        self.gl_buffer = BufferRenderer::new(gl, &self.raw_vertex_buffer, &self.raw_indices_buffer);
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        self.gl_buffer.cleanup(gl);
    }
}
