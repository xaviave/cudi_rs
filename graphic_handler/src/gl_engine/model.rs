use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
};

use glow::*;
use iced_glow::glow;
use media_handler::frame::Frame;
use obj::raw::material::{Material, MtlTextureMap};

use crate::gl_engine::material::CMaterial;

use super::gl_error::gl_error;
use super::textures::texture_util::TextureUtil;
use super::{buffer_renderer::BufferRenderer, scene::AbstractTexture};

pub struct Model {
    pub raw_vertex_buffer: Vec<f32>,
    pub raw_indices_buffer: Vec<i32>,

    pub material_data: CMaterial,
    pub ambient_map_path: Option<PathBuf>,
    pub diffuse_map_path: Option<PathBuf>,
    pub specular_map_path: Option<PathBuf>,
    // normal map
    pub bump_map_path: Option<PathBuf>,

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

        println!("[CUDI SPECIFIC] check the update media value after first render");

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

    fn bind_textures(
        &mut self,
        gl: &glow::Context,
        program: glow::NativeProgram,
        tx: &Sender<u8>,
        rx: &Receiver<Frame>,
        textures: &HashMap<PathBuf, AbstractTexture>,
    ) {
        let activate_texture =
            |map_path: &Option<PathBuf>, i: u32, texture_name: &str| match map_path {
                Some(t) => match textures.get(t) {
                    Some(review) => unsafe {
                        gl.active_texture(glow::TEXTURE0 + i);
                        gl.bind_texture(
                            glow::TEXTURE_2D,
                            match review {
                                AbstractTexture::cudi(mut x) => {
                                    tx.send(1).unwrap();
                                    x.update_media(gl, tx, rx, true, true);
                                    x.texture_ref
                                }
                                AbstractTexture::texture(x) => x.texture_ref,
                            },
                        );
                        gl.uniform_1_i32(
                            gl.get_uniform_location(program, texture_name).as_ref(),
                            i as i32,
                        );
                        gl_error(gl, String::from(format!("Error in texture {}", i)));
                    },
                    None => (),
                },
                _ => (),
            };

        activate_texture(&self.ambient_map_path, 0, "ambientMap");
        activate_texture(&self.diffuse_map_path, 1, "diffuseMap");
        activate_texture(&self.specular_map_path, 2, "specularMap");
        activate_texture(&self.bump_map_path, 3, "normalMap");
        unsafe {
            gl.active_texture(glow::TEXTURE0);
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        program: glow::NativeProgram,
        tx: &Sender<u8>,
        rx: &Receiver<Frame>,
        textures: &HashMap<PathBuf, AbstractTexture>,
    ) {
        self.material_data.update_material(gl, program);

        unsafe {
            // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
            gl.bind_vertex_array(Some(self.gl_buffer.vao));
            // self.bind_textures(gl, program, tx, rx, textures);

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
