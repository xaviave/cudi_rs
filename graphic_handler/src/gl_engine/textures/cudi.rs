use std::sync::mpsc::{Receiver, Sender};

use glow::*;
use iced_glow::glow;
use media_handler::frame::Frame;

use super::texture_util::TextureUtil;

#[derive(Copy, Clone)]
pub struct Cudi {
    pub ratio: f32,
    pub texture_ref: Option<glow::NativeTexture>,
}

impl TextureUtil for Cudi {}

impl Cudi {
    pub fn update_media(
        &mut self,
        gl: &glow::Context,
        tx: &Sender<u8>,
        rx: &Receiver<Frame>,
        need_refresh: bool,
        update_media: bool,
    ) {
        if need_refresh && update_media {
            let _ = tx.send(1);
            // let mut rng = rand::thread_rng();

            let new_media = match rx.recv() {
                Ok(f) => Some(f),
                Err(_) => None,
            };
            if let Some(m) = new_media {
                self.ratio = m.ratio;
                self.clean(gl);

                let texture_ref = Self::init_texture(gl);
                Self::generate_texture(gl, texture_ref, &m);
                self.texture_ref = Some(texture_ref);
                // Self::update_texture(gl, self.texture_ref.unwrap(), &m);
            }
            unsafe {
                gl.bind_texture(glow::TEXTURE_2D, self.texture_ref);
                // gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_renderer.fbo));
            }
        }
    }

    pub fn new(gl: &glow::Context, loading_media: &Frame) -> Self {
        let texture_ref = Self::init_texture(gl);
        Self::generate_texture(gl, texture_ref, loading_media);
        Self {
            ratio: loading_media.ratio,
            texture_ref: Some(texture_ref),
        }
    }

    pub fn clean(&mut self, gl: &glow::Context) {
        unsafe { gl.delete_texture(self.texture_ref.unwrap()) };
    }
}
