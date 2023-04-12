use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use rand;
use rand::Rng;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;

use crate::graphic_config::GraphicConfig;
use crate::scene::Scene;
use media_handler::frame::Frame;

use nalgebra_glm::{scale, translate, translation, vec3, TMat4, TVec3};

pub struct GlProgram {
    pub program: glow::Program,
    pub program_framebuffer: glow::Program,

    vao: glow::VertexArray,
    vbo: glow::NativeBuffer,
    quad_vao: glow::VertexArray,
    quad_vbo: glow::NativeBuffer,
    fbo: glow::NativeFramebuffer,
    color_texture_buffer: glow::NativeTexture,

    texture: glow::NativeTexture,
    scene: Scene,
    pub bg_color: Color,
}

impl GlProgram {
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

    pub fn new(gl: &glow::Context, config: &GraphicConfig, win_size: (i32, i32)) -> Self {
        unsafe {
            /*
                Create cudi program
                Create framebuffer program
                Create the main texture
                Create the render scene
            */
            let (program, vao, vbo) = Self::init_program_buffer(
                gl,
                &config.vertex_path,
                &config.fragment_path,
                &[3, 2],
                &Self::get_vertex_array(),
            );
            let (program_framebuffer, quad_vao, quad_vbo, fbo, color_texture_buffer) =
                Self::init_program_framebuffer(
                    gl,
                    &config.fbo_vertex_path,
                    &config.fbo_fragment_path,
                    win_size,
                );
            let texture = Self::init_texture(gl);
            Self::generate_texture(gl, texture, &config.loading_media);

            let mut scene = Scene::new(gl, &program);
            scene.ratio = config.loading_media.ratio;

            gl.use_program(Some(program));
            Self {
                program,
                program_framebuffer,
                vao,
                vbo,
                quad_vao,
                quad_vbo,
                fbo,
                color_texture_buffer,
                texture,
                scene,
                bg_color: Color::new(0., 0., 0., 1.),
            }
        }
    }

    pub fn draw(&mut self, gl: &glow::Context, media: Option<Frame>, viewport_ratio: f32) {
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
            //  1. Render in the framebuffer texture
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo));
            // don't clear the framebuffer GL_COLOR_BUFFER_BIT to keep last buffer data
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.enable(glow::DEPTH_TEST);

            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.use_program(Some(self.program));

            self.scene.update_scene(gl);

            gl.bind_vertex_array(Some(self.vao));
            for (_, position) in cubes_indices.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: TMat4<f32> =
                    translate(&translation(&position), &self.scene.last_pos);
                model = scale(
                    &model,
                    &vec3(self.scene.ratio * 0.5, viewport_ratio * 0.5, 1.),
                );
                self.scene.update_model(gl, model);
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
            // Unbind everything to clean
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);

            // 2. Bind default framebuffer, draw a plane and show the texture scene
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.disable(glow::DEPTH_TEST);

            gl.use_program(Some(self.program_framebuffer));
            gl.bind_vertex_array(Some(self.quad_vao));
            gl.bind_texture(glow::TEXTURE_2D, Some(self.color_texture_buffer));
            gl.draw_arrays(glow::TRIANGLES, 0, 6)
        }
    }

    pub fn resize_buffer(
        &mut self,
        gl: &glow::Context,
        win_size: (i32, i32),
        config: &GraphicConfig,
    ) {
        self.cleanup(gl);

        (self.program, self.vao, self.vbo) = Self::init_program_buffer(
            gl,
            &config.vertex_path,
            &config.fragment_path,
            &[3, 2],
            &Self::get_vertex_array(),
        );
        (
            self.program_framebuffer,
            self.quad_vao,
            self.quad_vbo,
            self.fbo,
            self.color_texture_buffer,
        ) = Self::init_program_framebuffer(
            gl,
            &config.fbo_vertex_path,
            &config.fbo_fragment_path,
            win_size,
        );
        self.texture = Self::init_texture(gl);

        // clear framebuffer that will be display
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo));
        }
        self.clear(gl);
    }

    pub fn clear(&self, gl: &glow::Context) {
        let [r, g, b, a] = self.bg_color.into_linear();
        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_program(self.program_framebuffer);
            gl.delete_vertex_array(self.vao);
            gl.delete_vertex_array(self.quad_vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.quad_vbo);
            gl.delete_framebuffer(self.fbo);
        }
    }
}
