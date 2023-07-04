use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use glow::*;
use iced_glow::glow;
use iced_glow::Color;
use media_handler::frame::Frame;
use nalgebra_glm::perspective;
use nalgebra_glm::rotation;
use obj::load_obj;
use obj::raw;
use obj::raw::object::Polygon;
use obj::Obj;

use crate::gl_engine::buffer_util::BufferUtil;
use crate::gl_engine::light::Light;
use crate::gl_engine::material::Material;
use crate::gl_engine::model::Model;
use crate::graphic_config::GraphicConfig;
use nalgebra_glm::{scale, translate, translation, vec3, TMat4, TVec3};

pub struct Scene {
    // metadata
    time: Instant,
    viewport_ratio: f32,
    update_media: bool,

    // fragment shader
    bg_color: Color,
    light_data: Light,

    // vertex shader
    pub models: Vec<Model>,
    last_position: TVec3<f32>,
    view_mat: TMat4<f32>,
    model_mat: TMat4<f32>,
    projection_mat: TMat4<f32>,

    // opengl
    program: glow::Program,
    model_loc: Option<NativeUniformLocation>,
    view_loc: Option<NativeUniformLocation>,
    projection_loc: Option<NativeUniformLocation>,
}
impl BufferUtil for Scene {}

impl Scene {
    pub fn new(
        gl: &glow::Context,
        index: u8,
        config: &GraphicConfig,
        viewport_ratio: f32,
        update_media: bool,
    ) -> Self {
        let program = Self::create_program(gl, &config.vertex_path, &config.fragment_path);

        let raw_obj = raw::parse_obj(BufReader::new(
            File::open(&config.scenes[index as usize]).unwrap(),
        ))
        .unwrap();

        let obj: Obj = load_obj(BufReader::new(
            File::open(&config.scenes[index as usize]).unwrap(),
        ))
        .unwrap();
        /*
            Create graphic program
            Create the render scene
        */
        let raw_indices: Vec<i32> = obj.indices.clone().into_iter().map(|e| e as i32).collect();
        // [x, y, z, nx, ny, nz, tx, ty, (tz)]
        let raw_vertex: Vec<f32> = obj
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

        // let mut models = vec![];
        // for (n, r) in &raw_obj.meshes {
        //     println!("{} | {:?}", n, r);

        //     let mut raw_indices: Vec<i32> = vec![];
        //     let mut raw_vertex: Vec<f32> = vec![];
        //     for x in &r.polygons {
        //         let a = &raw_obj.polygons[x.start..x.end];
        //         let idx = a
        //             .into_iter()
        //             .map(|v| match v {
        //                 Polygon::PTN(vec) => [vec[0].0, vec[1].0, vec[2].0],
        //                 _ => panic!(),
        //             })
        //             .collect::<Vec<_>>()
        //             .into_iter()
        //             .flatten()
        //             .collect::<Vec<_>>();
        //         for i in idx {
        //             raw_indices.push(obj.indices[i] as i32);
        //             raw_vertex.append(&mut {
        //                 let v = obj.vertices[i];
        //                 vec![
        //                     v.position[0],
        //                     v.position[1],
        //                     v.position[2],
        //                     v.normal[0],
        //                     v.normal[1],
        //                     v.normal[2],
        //                     0.,
        //                     0.,
        //                     // 0.,
        //                     // v.texture[0],
        //                     // v.texture[1],
        //                     // v.texture[2],
        //                 ]
        //             });
        //         }
        //     }
        //     models.push(Model::new(
        //         gl,
        //         program,
        //         raw_vertex,
        //         raw_indices,
        //         Material::new(gl, program),
        //         update_media,
        //     ));
        // }
        let models = Model::new(
            gl,
            program,
            raw_vertex,
            raw_indices.to_vec(),
            Material::new(gl, program),
            update_media,
        );
        unsafe {
            Self {
                time: Instant::now(),
                viewport_ratio,
                update_media,

                bg_color: Color::new(0., 0., 0., 1.),
                light_data: Light::new(gl, &program),

                models: vec![models],
                last_position: vec3(0., 0., 0.),
                model_mat: translation(&(vec3(0., -1., -5.)))
                    * rotation(90.0_f32.to_radians(), &(vec3(0., 1.0, 0.0).normalize())),
                view_mat: translation(&(vec3(0., 0., -3.).normalize())),
                projection_mat: perspective(viewport_ratio, (45_f32).to_radians(), 0.1, 100.0),

                program,
                model_loc: gl.get_uniform_location(program, "model"),
                view_loc: gl.get_uniform_location(program, "view"),
                projection_loc: gl.get_uniform_location(program, "projection"),
            }
        }
    }

    fn update_matrix(&mut self, gl: &glow::Context, ux_value: Color) {
        let mut model: TMat4<f32> = translate(
            &translation(&self.last_position),
            &vec3(ux_value.r, ux_value.g, ux_value.b),
        );
        model *= self.model_mat
            * rotation(
                self.time.elapsed().as_millis() as f32 * 0.05_f32.to_radians(),
                &vec3(0.0, 1.0, 0.0),
            );
        model = scale(
            &model,
            &vec3(0.5, 0.5, 0.5),
            // used for rectangle that need to scale exactly like an image
            // maybe implement an impl for image-scene
            // &vec3(self.scene.ratio * 0.1, viewport_ratio * 0.1, 1.).normalize(),
        );

        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(self.model_loc.as_ref(), false, model.as_slice());
            gl.uniform_matrix_4_f32_slice(self.view_loc.as_ref(), false, self.view_mat.as_slice());
            gl.uniform_matrix_4_f32_slice(
                self.projection_loc.as_ref(),
                false,
                self.projection_mat.as_slice(),
            );
        }
    }

    fn update_scene_data(&mut self, gl: &glow::Context) {
        self.light_data.update_light(gl, self.program);
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
        // let cubes_indices: [TVec3<f32>; 3] = [
        //     vec3(0.0, 0.0, -5.0),
        //     vec3(0.0, 3.0, -5.0),
        //     vec3(0.0, -3.0, -5.0),
        // ];

        self.update_scene_data(gl);
        self.update_matrix(gl, ux_value);
        for m in &mut self.models {
            m.draw(gl, self.program, rx);
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        for m in &mut self.models {
            m.cleanup(gl);
        }
        unsafe {
            gl.delete_program(self.program);
        }
    }
}
