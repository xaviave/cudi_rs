use std::fs;
use std::path::PathBuf;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;

pub struct GlProgram {
    program: glow::Program,
    vertex_array: glow::VertexArray,
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
        unsafe {
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

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
                vertex_array,
            }
        }
    }

    pub fn clear(&self, gl: &glow::Context, background_color: Color) {
        let [r, g, b, a] = background_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.use_program(Some(self.program));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }
}
