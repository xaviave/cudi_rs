use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use glow::*;
use iced_glow::glow;

pub trait BufferUtil {
    fn get_vertex_array() -> [f32; 30] {
        [
            // upper left triangle
            -1.0, 1.0, 0.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
            // lower right triangle
            1.0, 1.0, 0.0, 1.0, 1.0, 1.0, -1.0, 0.0, 1.0, 0.0, -1.0, -1.0, 0.0, 0.0, 0.0,
        ]
    }

    fn get_shader_from_file(shader_path: PathBuf) -> String {
        fs::read_to_string(shader_path).expect("Unable to read file")
    }

    fn init_shaders(
        gl: &glow::Context,
        program: glow::NativeProgram,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
    ) -> Vec<NativeShader> {
        /*
            Open and read vertex and fragment shader from file
            Create the NativeShader from String, compile and attach the shaders to the GL program
            Return the shaders to clean them after the program is linked to the GL context
        */
        let shader_version = "#version 410";
        let shader_sources = [
            (
                glow::VERTEX_SHADER,
                Self::get_shader_from_file(vertex_path.to_path_buf()),
            ),
            (
                glow::FRAGMENT_SHADER,
                Self::get_shader_from_file(fragment_path.to_path_buf()),
            ),
        ];
        let mut shaders = Vec::with_capacity(shader_sources.len());

        unsafe {
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }
        }
        shaders
    }

    fn create_program(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
    ) -> glow::NativeProgram {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");
            let shaders = Self::init_shaders(gl, program, vertex_path, fragment_path);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
            program
        }
    }

    fn init_buffers(
        gl: &glow::Context,
        byte_sizes: &[i32],
        vertices: &[f32],
    ) -> (glow::NativeVertexArray, glow::NativeBuffer) {
        // Vertex buffer attributes
        let mut offset = 0;
        let size_f32 = size_of::<f32>() as i32;
        let stride = byte_sizes.iter().sum::<i32>() * size_f32;

        unsafe {
            // Vertex Buffer
            let vbo = gl.create_buffer().expect("Cannot create buffer");
            let (_, vertices_bytes, _) = vertices.align_to::<u8>();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_bytes, glow::STATIC_DRAW);

            // Vertex Array
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            gl.bind_vertex_array(Some(vao));
            for (i, size) in byte_sizes.iter().enumerate() {
                gl.enable_vertex_attrib_array(i as u32);
                gl.vertex_attrib_pointer_f32(
                    i as u32,
                    *size,
                    glow::FLOAT,
                    false,
                    stride,
                    offset * size_f32,
                );
                offset += size;
            }
            gl.bind_vertex_array(None);

            (vao, vbo)
        }
    }

    fn init_program_buffer(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
        byte_sizes: &[i32],
        vertices: &[f32],
    ) -> (NativeProgram, NativeVertexArray, NativeBuffer) {
        /*
        Create the shaders and link them to the program
        Create the buffers
        */
        let program = Self::create_program(gl, vertex_path, fragment_path);
        let (vao, vbo) = Self::init_buffers(gl, byte_sizes, vertices);
        (program, vao, vbo)
    }
}
