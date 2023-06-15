use std::slice;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;
use media_handler::frame::Frame;
use nalgebra_glm::rotation;
use nalgebra_glm::Vec3;

use crate::gl_engine::buffer_util::BufferUtil;
use crate::gl_engine::scene::Scene;
use crate::gl_engine::texture_util::TextureUtil;
use crate::graphic_config::GraphicConfig;
use nalgebra_glm::{scale, translate, translation, vec3, TMat4, TVec3};

pub struct BufferRenderer {
    pub vao: glow::VertexArray,
    pub ebo: glow::NativeBuffer,
    pub vbo: glow::NativeBuffer,
    pub program: glow::Program,

    pub scene: Scene,
    pub bg_color: Color,
    texture: glow::NativeTexture,
    pub update_media: bool,
    time: Instant,
}
impl TextureUtil for BufferRenderer {}
impl BufferUtil for BufferRenderer {}

impl BufferRenderer {
    pub fn new(
        gl: &glow::Context,
        index: u8,
        config: &GraphicConfig,
        viewport_ratio: f32,
        update_media: bool,
    ) -> Self {
        /*
            Create graphic program
            Create the render scene
        */
        let raw_indices: Vec<i32> = config.scenes[index as usize]
            .indices
            .clone()
            .into_iter()
            .map(|e| e as i32)
            .collect();
        // [x, y, z, nx, ny, nz, tx, ty, (tz)]
        let raw_vertex: Vec<f32> = config.scenes[index as usize]
            .vertices
            .clone()
            .into_iter()
            .map(|v| {
                vec![
                    v.position[0],
                    v.position[1],
                    v.position[2],
                    v.normal[0],
                    v.normal[1],
                    v.normal[2],
                    0.,
                    0.,
                    // 0.,
                    // v.texture[0],
                    // v.texture[1],
                    // v.texture[2],
                ]
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect();

        // should open the mtl file here

        let (program, vao, vbo) = Self::init_program_buffer(
            gl,
            &config.vertex_path,
            &config.fragment_path,
            &[3, 3, 2],
            unsafe { slice::from_raw_parts(raw_vertex.as_ptr(), raw_vertex.len()) },
        );

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

        let scene = Scene::new(
            gl,
            &program,
            viewport_ratio,
            raw_vertex,
            raw_indices.to_vec(),
        );
        scene.update_model(gl, scene.model);
        scene.update_scene(gl);

        println!("check the update media value after first render");
        Self {
            vao,
            ebo,
            vbo,
            program,
            scene,
            bg_color: Color::new(0., 0., 0., 1.),
            texture: Self::init_texture(gl),
            update_media,
            time: Instant::now(),
        }
    }

    pub fn update_scene_data(&mut self, last_pos: Vec3) {
        self.scene.last_pos = last_pos;
    }

    fn set_media_texture(&mut self, gl: &glow::Context, rx: &Receiver<Frame>, need_refresh: bool) {
        if need_refresh && self.update_media {
            let new_media = match rx.recv() {
                Ok(f) => Some(f),
                Err(_) => None,
            };
            if let Some(m) = new_media {
                self.scene.texture_ratio = m.ratio;
                Self::generate_texture(gl, self.texture, &m);
            }
            unsafe {
                gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            }
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        rx: &Receiver<Frame>,
        need_refresh: bool,
        ux_value: Color,
    ) {
        // should handle the media here
        if !self.update_media && !need_refresh {
            return;
        }
        let cubes_indices: [TVec3<f32>; 3] = [
            vec3(0.0, 0.0, -5.0),
            vec3(0.0, 3.0, -5.0),
            vec3(0.0, -3.0, -5.0),
        ];

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            let mut mask = glow::DEPTH_BUFFER_BIT;
            if !self.update_media {
                mask |= glow::COLOR_BUFFER_BIT;
                gl.clear_color(0., 0., 0., 0.);
            }
            gl.clear(mask);

            self.set_media_texture(gl, rx, need_refresh);
            gl.use_program(Some(self.program));
            self.scene.update_scene(gl);

            gl.bind_vertex_array(Some(self.vao));
            for (i, position) in cubes_indices.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: TMat4<f32> = translate(
                    &translation(&position),
                    &vec3(ux_value.r, ux_value.g, ux_value.b),
                );
                model *= self.scene.model
                    * rotation(
                        self.time.elapsed().as_millis() as f32
                            * 0.05_f32.to_radians()
                            * (i + 1) as f32,
                        &vec3(0.0, 1.0, 0.0),
                    );

                model = scale(
                    &model,
                    &vec3(0.5, 0.5, 0.5),
                    // used for rectangle that need to scale exactly like an image
                    // maybe implement an impl for image-scene
                    // &vec3(self.scene.ratio * 0.1, viewport_ratio * 0.1, 1.).normalize(),
                );
                self.scene.update_model(gl, model);
                // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
                gl.draw_elements(
                    glow::TRIANGLES,
                    self.scene.raw_indices_buffer.len() as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            }

            // Unbind everything to clean
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn cleanup(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.texture);
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }
}
