use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;
use media_handler::Frame;

// use nalgebra;
use nalgebra_glm::{perspective, rotation, translation, vec3, TMat4, TVec3};

pub struct GlProgram {
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: NativeBuffer,
    texture: NativeTexture,
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

    pub fn new(
        gl: &glow::Context,
        vertex_path: &PathBuf,
        fragment_path: &PathBuf,
        loading_media: &Frame,
    ) -> Self {
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

            let vertices: [f32; 180] = [
                -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5,
                0.5, 0.5, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5,
                0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5,
                -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5,
                1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5,
                0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0,
                1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5,
                0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
            ];
            // Vertex Buffer
            let vbo = gl.create_buffer().unwrap();
            let (_, vertices_bytes, _) = vertices.align_to::<u8>();
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, vertices_bytes, STATIC_DRAW);

            // Vertex Array
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));
            // Vertex buffer attributes
            let mut offset = 0;
            let sizes = [3, 2];
            let size_f32 = size_of::<f32>() as i32;
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
            // gl.bind_vertex_array(None);

            let texture = gl.create_texture().unwrap();
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

            gl.tex_image_2d(
                TEXTURE_2D,
                0,
                RGBA as i32,
                loading_media.width as i32,
                loading_media.height as i32,
                0,
                RGBA,
                UNSIGNED_BYTE,
                Some(&loading_media.get_raw_image()),
            );
            gl.generate_mipmap(TEXTURE_2D);

            gl.use_program(Some(program));
            Self {
                program,
                vao,
                vbo,
                texture,
            }
        }
    }

    pub unsafe fn clear(&self, gl: &glow::Context, background_color: Color) {
        let [r, g, b, a] = background_color.into_linear();
        gl.clear_color(r, g, b, a);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    unsafe fn generate_texture(&self, gl: &glow::Context, media: &Frame) {
        // let t = gl.create_texture().expect("Error while creating texture");
        gl.bind_texture(TEXTURE_2D, Some(self.texture));

        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGBA as i32,
            media.width as i32,
            media.height as i32,
            0,
            RGBA,
            UNSIGNED_BYTE,
            Some(&media.get_raw_image()),
        );
        gl.generate_mipmap(TEXTURE_2D);
    }

    pub unsafe fn draw(&self, gl: &glow::Context, media: Option<Frame>) {
        let cubes_indices: [TVec3<f32>; 10] = [
            vec3(0.0, 0.0, 0.0),
            vec3(2.0, 5.0, -15.0),
            vec3(-1.5, -2.2, -2.5),
            vec3(-3.8, -2.0, -12.3),
            vec3(2.4, -0.4, -3.5),
            vec3(-1.7, 3.0, -7.5),
            vec3(1.3, -2.0, -2.5),
            vec3(1.5, 2.0, -2.5),
            vec3(1.5, 0.2, -1.5),
            vec3(-1.3, 1.0, -1.5),
        ];
        if let Some(m) = media {
            self.generate_texture(gl, &m);
        }
        gl.bind_texture(TEXTURE_2D, Some(self.texture));

        gl.use_program(Some(self.program));

        let model: TMat4<f32> =
            rotation((0.0_f32).to_radians(), &(vec3(0.5, 1.0, 0.0).normalize()));
        let view: TMat4<f32> = translation(&(vec3(0., 0., -3.).normalize()));
        let projection: TMat4<f32> = perspective(1., (45_f32).to_radians(), 0.1, 100.0);
        let model_loc = gl.get_uniform_location(self.program, "model");
        let view_loc = gl.get_uniform_location(self.program, "view");
        let projection_loc = gl.get_uniform_location(self.program, "projection");

        gl.uniform_matrix_4_f32_slice(model_loc.as_ref(), false, model.as_slice());
        gl.uniform_matrix_4_f32_slice(view_loc.as_ref(), false, view.as_slice());
        gl.uniform_matrix_4_f32_slice(projection_loc.as_ref(), false, projection.as_slice());

        gl.bind_vertex_array(Some(self.vao));
        for (i, position) in cubes_indices.iter().enumerate() {
            // calculate the model matrix for each object and pass it to shader before drawing
            let angle = (20 * i) as f32;
            let mut model: TMat4<f32> = translation(&position);
            model = model * rotation(angle.to_radians(), &vec3(1.0, 0.3, 0.5));
            gl.uniform_matrix_4_f32_slice(model_loc.as_ref(), false, model.as_slice());
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
        gl.bind_vertex_array(None);
        gl.bind_texture(TEXTURE_2D, None);
    }

    pub unsafe fn cleanup(&self, gl: &glow::Context) {
        gl.delete_program(self.program);
        gl.delete_vertex_array(self.vao);
        gl.delete_buffer(self.vbo)
    }
}
