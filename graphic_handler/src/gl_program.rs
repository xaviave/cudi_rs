use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use rand;
use rand::Rng;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;

use crate::scene::Scene;
use media_handler::Frame;

use nalgebra_glm::{scale, translate, translation, vec3, TMat4, TVec3};

pub struct GlProgram {
    pub program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::NativeBuffer,
    texture: glow::NativeTexture,
    scene: Scene,
}

impl GlProgram {
    fn get_vertex_array() -> [f32; 30] {
        [
            // upper left triangle
            -0.5, 0.5, 0.0, 0.0, 1.0, -0.5, -0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 1.0, 1.0,
            // lower right triangle
            0.5, 0.5, 0.0, 1.0, 1.0, 0.5, -0.5, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
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

    fn init_buffers(gl: &glow::Context) -> (NativeVertexArray, NativeBuffer) {
        // Vertex buffer attributes
        let mut offset = 0;
        let sizes = [3, 2];
        let size_f32 = size_of::<f32>() as i32;
        let stride = sizes.iter().sum::<i32>() * size_f32;
        let vertices: [f32; 30] = Self::get_vertex_array();

        unsafe {
            // Vertex Buffer
            let vbo = gl.create_buffer().unwrap();
            let (_, vertices_bytes, _) = vertices.align_to::<u8>();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_bytes, glow::STATIC_DRAW);

            // Vertex Array
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            gl.bind_vertex_array(Some(vao));
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
            (vao, vbo)
        }
    }

    fn init_texture(gl: &glow::Context) -> NativeTexture {
        unsafe {
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            texture
        }
    }

    fn generate_texture(gl: &glow::Context, texture: NativeTexture, media: &Frame) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                RGBA as i32,
                media.width as i32,
                media.height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&media.get_raw_image()),
            );
            gl.generate_mipmap(glow::TEXTURE_2D);
        }
    }

    pub fn new(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
        loading_media: &Frame,
    ) -> Self {
        unsafe {
            /*
                Create program
                Create the shaders and link them to the program
                Create the buffers
                Create the main texture
                Create the render scene
            */
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

            let (vao, vbo) = Self::init_buffers(gl);
            let texture = Self::init_texture(gl);
            Self::generate_texture(gl, texture, loading_media);

            let mut scene = Scene::new(gl, &program);
            scene.ratio = loading_media.ratio;

            gl.use_program(Some(program));
            Self {
                program,
                vao,
                vbo,
                texture,
                scene,
            }
        }
    }

    pub fn clear(&self, gl: &glow::Context, background_color: Color) {
        let [r, g, b, a] = background_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&mut self, gl: &glow::Context, media: Option<Frame>) {
        let mut rng = rand::thread_rng();
        let cubes_indices: [TVec3<f32>; 1] = [vec3(0.0, 0.0, -3.0)];

        if let Some(m) = media {
            self.scene.ratio = m.ratio;
            self.scene.last_pos = vec3(
                rng.gen_range(-1.2..1.2),
                rng.gen_range(-1.2..1.2),
                rng.gen_range(-1.2..1.2),
            );
            Self::generate_texture(gl, self.texture, &m);
        }

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.use_program(Some(self.program));

            self.scene.update_scene(gl);

            gl.bind_vertex_array(Some(self.vao));

            for (i, position) in cubes_indices.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: TMat4<f32> =
                    translate(&translation(&position), &self.scene.last_pos);
                model = scale(&model, &vec3(self.scene.ratio, 1.0, 1.));
                self.scene.update_model(gl, model);
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
            // Unbind everything to clean
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }
}
