use std::path::PathBuf;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;

use crate::gl_engine::buffer_util::BufferUtil;
use crate::gl_engine::textures::texture_util::TextureUtil;

pub struct FramebufferRenderer {
    pub vao: glow::VertexArray,
    pub vbo: glow::NativeBuffer,
    pub program: glow::Program,

    pub fbo: glow::NativeFramebuffer,
    pub color_texture_buffer: glow::NativeTexture,
    pub bg_color: Color,
}

impl BufferUtil for FramebufferRenderer {}
impl TextureUtil for FramebufferRenderer {}

impl FramebufferRenderer {
    fn init_program_framebuffer(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
        win_size: (i32, i32),
    ) -> (
        glow::NativeProgram,
        glow::NativeVertexArray,
        glow::NativeBuffer,
        glow::NativeFramebuffer,
        NativeTexture,
    ) {
        let byte_sizes = [2, 2];
        let vertices: [f32; 24] = [
            -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
        ];

        unsafe {
            let (program, vao, vbo) =
                Self::init_program_buffer(gl, vertex_path, fragment_path, &byte_sizes, &vertices);

            let fbo = gl.create_framebuffer().expect("Cannot create framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));

            let color_texture_buffer = Self::init_texture(gl);
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as i32,
                win_size.0,
                win_size.1,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                None,
            );
            gl.bind_texture(glow::TEXTURE_2D, None);
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(color_texture_buffer),
                0,
            );
            let rbo = gl
                .create_renderbuffer()
                .expect("Cannot create render buffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));
            gl.renderbuffer_storage(
                glow::RENDERBUFFER,
                glow::DEPTH24_STENCIL8,
                win_size.0,
                win_size.1,
            );
            gl.bind_renderbuffer(glow::RENDERBUFFER, None);

            gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::DEPTH_STENCIL_ATTACHMENT,
                glow::RENDERBUFFER,
                Some(rbo),
            );
            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                panic!("Fail to bind framebuffer");
            }
            // render in main window
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            (program, vao, vbo, fbo, color_texture_buffer)
        }
    }

    pub fn new(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
        win_size: (i32, i32),
    ) -> Self {
        /*
            Create main program where all the other program will render in
        */
        let (program, vao, vbo, fbo, color_texture_buffer) =
            Self::init_program_framebuffer(gl, vertex_path, fragment_path, win_size);

        Self {
            program,
            vao,
            vbo,
            fbo,
            color_texture_buffer,
            bg_color: Color::new(0., 0., 0., 1.),
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            // Bind default framebuffer, draw a plane and show the texture scene
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.disable(glow::DEPTH_TEST);
            gl.clear(glow::DEPTH_BUFFER_BIT | glow::COLOR_BUFFER_BIT);

            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_texture(glow::TEXTURE_2D, Some(self.color_texture_buffer));
            gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
            gl.draw_arrays(glow::TRIANGLES, 0, 6)
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_framebuffer(self.fbo);
        }
    }
}
