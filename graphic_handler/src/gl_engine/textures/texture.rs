use std::path::PathBuf;

use glow::*;
use iced_glow::glow;
use media_handler::frame::Frame;

use super::texture_util::TextureUtil;
#[derive(Copy, Clone)]
pub struct Texture {
    pub texture_ref: Option<glow::NativeTexture>,
}
impl TextureUtil for Texture {}

impl Texture {
    pub fn new(gl: &glow::Context, tex_file: PathBuf) -> Self {
        let texture_ref = Self::init_texture(gl);
        let media = Frame::new(tex_file);
        Self::generate_texture(gl, texture_ref, &media);
        Self {
            texture_ref: Some(texture_ref),
        }
    }

    pub fn clean(&mut self, gl: &glow::Context) {
        unsafe { gl.delete_texture(self.texture_ref.unwrap()) };
    }
}
