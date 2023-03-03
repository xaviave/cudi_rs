use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;

pub struct GlProgram {
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: NativeBuffer,
    ebo: NativeBuffer,
}

impl GlProgram {
    fn get_shader_from_file(shader_path: PathBuf) -> String {
        fs::read_to_string(shader_path).expect("Unable to read file")
    }

    fn init_shaders(
        gl: &glow::Context,
        program: NativeProgram,
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

    pub fn new(gl: &glow::Context, vertex_path: &PathBuf, fragment_path: &PathBuf) -> Self {
        let vertices: [f32; 32] = [
            // positions          // colors           // texture coords
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
            -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
        ];
        let indices: [i32; 6] = [
            0, 1, 3, // first Triangle
            1, 2, 3, // second Triangle
        ];
        // https://github.com/willcrichton/learn-opengl-rust

        unsafe {
            // Vertex Array
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));

            // Index Buffer
            let ebo = gl.create_buffer().unwrap();
            let (_, indices_bytes, _) = indices.align_to::<u8>();
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, indices_bytes, STATIC_DRAW);

            // Vertex Buffer
            let vbo = gl.create_buffer().unwrap();
            let (_, vertices_bytes, _) = vertices.align_to::<u8>();
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, vertices_bytes, STATIC_DRAW);

            let mut offset = 0;

            // Vertex buffer attributes
            let size_f32 = size_of::<f32>() as i32;
            let sizes = [3, 3, 2];
            let stride = sizes.iter().sum::<i32>() * size_f32;

            for (i, size) in sizes.iter().enumerate() {
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

            gl.use_program(Some(program));
            Self {
                program,
                vao,
                vbo,
                ebo,
            }
        }
    }

    pub unsafe fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
        let uniform_location = gl.get_uniform_location(program, name);
        // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
        gl.uniform_1_f32(uniform_location.as_ref(), value)
    }

    pub fn clear(&self, gl: &glow::Context, background_color: Color) {
        let [r, g, b, a] = background_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        let indices: [i32; 6] = [
            0, 1, 3, // first Triangle
            1, 2, 3, // second Triangle
        ];
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.use_program(Some(self.program));
            gl.draw_elements(glow::TRIANGLES, indices.len() as i32, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo)
        }
    }
}
