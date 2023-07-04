use std::slice;

use glow::*;
use iced_glow::glow;

use crate::gl_engine::buffer_util::BufferUtil;

pub struct BufferRenderer {
    pub vao: glow::VertexArray,
    pub ebo: glow::NativeBuffer,
    pub vbo: glow::NativeBuffer,
}
impl BufferUtil for BufferRenderer {}

impl BufferRenderer {
    pub fn new(gl: &glow::Context, raw_vertices: &[f32], raw_indices: &[i32]) -> Self {
        /*
            Create graphic program
            Create the render scene
        */
        let (vao, vbo) = Self::init_buffers(gl, &[3, 3, 2], unsafe {
            slice::from_raw_parts(raw_vertices.as_ptr(), raw_vertices.len())
        });

        let ebo = unsafe {
            // Index Buffer
            gl.bind_vertex_array(Some(vao));
            let ebo = gl.create_buffer().unwrap();
            let (_, indices_bytes, _) = raw_indices.align_to::<u8>();
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, indices_bytes, STATIC_DRAW);
            gl.bind_vertex_array(None);
            ebo
        };

        println!("check the update media value after first render");
        Self { vao, ebo, vbo }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
        }
    }
}
