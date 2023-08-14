use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use glow::*;
use iced_glow::glow;
use iced_winit::winit::event::VirtualKeyCode;
use media_handler::frame::Frame;
use obj::raw;
use obj::raw::object::Polygon;
use obj::raw::RawMtl;
use obj::raw::RawObj;

use crate::controls::Controls;
use crate::gl_engine::buffer_util::BufferUtil;
use crate::gl_engine::lights::directional_light::DirectionalLight;
use crate::gl_engine::model::Model;
use crate::graphic_config::GraphicConfig;
use nalgebra_glm::{
    mat3, mat3_to_mat4, perspective, scale, translate, translation, vec3, TMat4, TVec3, Vec3,
};

use super::camera::Camera;
use super::lights::point_light::PointLight;
use super::lights::spot_light::SpotLight;
use super::texture_util::TextureUtil;

pub struct Scene {
    // metadata
    time: Instant,
    pub fov: f32,
    viewport_ratio: f32,
    update_media: bool,

    // fragment shader
    directional_light_data: DirectionalLight,
    spot_light_data: Vec<SpotLight>,
    point_light_data: Vec<PointLight>,

    // vertex shader
    pub models: Vec<Model>,
    last_position_model: TVec3<f32>,
    model_mat: TMat4<f32>,
    camera: Camera,
    projection_mat: TMat4<f32>,

    // opengl
    program: glow::Program,
    time_loc: Option<NativeUniformLocation>,
    debug_loc: Option<NativeUniformLocation>,
    model_loc: Option<NativeUniformLocation>,
    view_loc: Option<NativeUniformLocation>,
    projection_loc: Option<NativeUniformLocation>,
    spot_light_number_loc: Option<NativeUniformLocation>,
    point_light_number_loc: Option<NativeUniformLocation>,

    textures: HashMap<PathBuf, Option<glow::NativeTexture>>,
}
impl BufferUtil for Scene {}
impl TextureUtil for Scene {}

impl Scene {
    fn parse_model(
        gl: &glow::Context,
        program: NativeProgram,
        raw_obj: RawObj,
        raw_material: &RawMtl,
        mtl_path: &Path,
        update_media: bool,
    ) -> Vec<Model> {
        let mut vertex_cache = HashMap::new();
        let mut map_data = |pi: usize,
                            ni: usize,
                            ti: usize,
                            raw_vertex: &mut Vec<f32>,
                            raw_indices: &mut Vec<i32>| {
            // Look up cache
            let index = match vertex_cache.entry((pi, ni, ti)) {
                // Cache miss -> make new, store it on cache
                Entry::Vacant(entry) => {
                    let p = raw_obj.positions[pi];
                    let n = raw_obj.normals[ni];
                    let t = raw_obj.tex_coords[ti];
                    let vertex = vec![
                        p.0, p.1, p.2, //
                        n.0, n.1, n.2, //
                        t.0, t.1, // t.2
                    ];
                    let index = raw_vertex.len() / 8;
                    raw_vertex.extend(vertex);
                    entry.insert(index);
                    index
                }
                // Cache hit -> use it
                Entry::Occupied(entry) => *entry.get(),
            };
            raw_indices.push(index as i32);
        };

        let mut models = vec![];
        for (n, r) in &raw_obj.meshes {
            let mut raw_indices: Vec<i32> = vec![];
            let mut raw_vertex: Vec<f32> = vec![];
            for x in r.polygons.iter() {
                let polygons = &raw_obj.polygons[x.start..x.end];
                for p in polygons.into_iter() {
                    match p {
                        Polygon::PTN(vec) => {
                            for &(pi, ti, ni) in vec {
                                map_data(pi, ni, ti, &mut raw_vertex, &mut raw_indices);
                            }
                        }
                        _ => panic!(),
                    }
                }
            }

            models.push(Model::new(
                gl,
                program,
                raw_vertex,
                raw_indices,
                update_media,
                mtl_path,
                &raw_material.materials[n],
            ));
        }
        models
    }

    fn parse_textures(
        gl: &glow::Context,
        raw_material: &RawMtl,
        mtl_path: &Path,
    ) -> HashMap<PathBuf, Option<glow::NativeTexture>> {
        let file_to_tex = |tex_file: PathBuf| -> Option<glow::NativeTexture> {
            let texture = Self::init_texture(gl);
            let media = Frame::new(tex_file);
            Self::generate_texture(gl, texture, &media);
            Some(texture)
        };

        let textures_paths: Vec<_> = raw_material
            .materials
            .values()
            .flat_map(|r| {
                [
                    r.ambient_map.as_ref(),
                    r.diffuse_map.as_ref(),
                    r.specular_map.as_ref(),
                    r.bump_map.as_ref(),
                ]
            })
            .filter_map(|f| f.map(|map| mtl_path.join(&map.file)))
            .fold(Vec::new(), |mut acc, f_map| {
                if !acc.contains(&f_map) {
                    acc.push(f_map);
                }
                acc
            });

        let mut textures_cache = HashMap::new();
        let mut map_data = |texture_name: PathBuf| {
            match textures_cache.entry(texture_name.clone()) {
                Entry::Vacant(entry) => {
                    let x = file_to_tex(texture_name);
                    entry.insert(x);
                    x
                }
                Entry::Occupied(entry) => *entry.get(),
            };
        };

        for p in textures_paths {
            map_data(p);
        }
        textures_cache
    }

    pub fn new(
        gl: &glow::Context,
        index: u8,
        config: &GraphicConfig,
        viewport_size: (f64, f64),
        update_media: bool,
        mouse_position: (f64, f64),
    ) -> Self {
        /*
            Create graphic program
            Create the render scene
        */

        let fov = 45.0;
        let viewport_ratio = (viewport_size.0 / viewport_size.1) as f32;
        let raw_obj = raw::parse_obj(BufReader::new(
            File::open(&config.scenes[index as usize]).unwrap(),
        ))
        .unwrap();

        let mtl_path = Path::new(&config.scenes[index as usize]).parent().unwrap();
        let raw_material = raw::parse_mtl(BufReader::new(
            File::open(mtl_path.join(&raw_obj.material_libraries[0])).unwrap(),
        ))
        .unwrap();

        let program = Self::create_program(gl, &config.vertex_path, &config.fragment_path);

        let models = Self::parse_model(gl, program, raw_obj, &raw_material, mtl_path, update_media);
        let textures = Self::parse_textures(gl, &raw_material, mtl_path);

        unsafe {
            gl.use_program(Some(program));
            Self {
                time: Instant::now(),
                fov,
                viewport_ratio,
                update_media,

                directional_light_data: DirectionalLight::new(
                    gl,
                    &program,
                    String::from("directional_light"),
                    0.2,
                ),
                spot_light_data: (0..3)
                    .map(|i| {
                        SpotLight::new(
                            gl,
                            &program,
                            format!("spot_lights[{}]", i),
                            vec3(0., 2., 2. * i as f32),
                        )
                    })
                    .collect(),
                point_light_data: (0..1)
                    .map(|i| {
                        PointLight::new(
                            gl,
                            &program,
                            format!("point_lights[{}]", i),
                            vec3(0., 2., -2.),
                        )
                    })
                    .collect(),

                models,
                last_position_model: vec3(0., 0., 0.),
                model_mat: translation(&(vec3(0., -1., -5.))),
                camera: Camera::new(
                    vec3(1.0, 1.0, -3.0),
                    vec3(0.0, 0.0, -1.0),
                    viewport_size,
                    mouse_position,
                ),
                projection_mat: perspective(viewport_ratio, (fov).to_radians(), 0.1, 100.0),

                program,
                time_loc: gl.get_uniform_location(program, "time"),
                debug_loc: gl.get_uniform_location(program, "debug"),
                model_loc: gl.get_uniform_location(program, "model"),
                view_loc: gl.get_uniform_location(program, "view"),
                projection_loc: gl.get_uniform_location(program, "projection"),
                spot_light_number_loc: gl.get_uniform_location(program, "spot_light_number"),
                point_light_number_loc: gl.get_uniform_location(program, "point_light_number"),
                textures,
            }
        }
    }

    pub fn update_projection(&mut self, viewport_size: (f64, f64), fov: f32) {
        self.fov = fov;
        self.camera.viewport_size = viewport_size;
        self.projection_mat = perspective(
            (viewport_size.0 / viewport_size.1) as f32,
            (fov).to_radians(),
            0.1,
            100.0,
        );
    }

    fn rotation_matrix(theta: Vec3) -> TMat4<f32> {
        // return a mat4 with a mat3 Eulers matrix used with theta angles
        // theta[0]: x axis
        // theta[1]: y axis
        // theta[2]: z axis
        let r_x = mat3(
            1.,
            0.,
            0.,
            0.,
            theta[0].cos(),
            -theta[0].sin(),
            0.,
            theta[0].sin(),
            theta[0].cos(),
        );
        let r_y = mat3(
            theta[1].cos(),
            0.,
            theta[1].sin(),
            0.,
            1.,
            0.,
            -theta[1].sin(),
            0.,
            theta[1].cos(),
        );
        let r_z = mat3(
            theta[2].cos(),
            -theta[2].sin(),
            0.,
            theta[2].sin(),
            theta[2].cos(),
            0.,
            0.,
            0.,
            1.,
        );
        mat3_to_mat4(&(r_z * (r_y * r_x)))
    }

    fn update_matrix(
        &mut self,
        gl: &glow::Context,
        ux_data: &Controls,
        keyboard_data: &Vec<VirtualKeyCode>,
        mouse_position: (f64, f64),
    ) {
        let mut model: TMat4<f32> = translate(
            &translation(&self.last_position_model),
            &ux_data.scene_position,
        );

        model *= self.model_mat * Self::rotation_matrix(ux_data.scene_rotation);
        model = scale(
            &model,
            &ux_data.scene_scale,
            // used for rectangle that need to scale exactly like an image
            // maybe implement an impl for image-scene
            // &vec3(self.scene.ratio * 0.1, viewport_ratio * 0.1, 1.).normalize(),
        );

        self.camera.update_camera(keyboard_data, mouse_position);
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(self.model_loc.as_ref(), false, model.as_slice());
            gl.uniform_matrix_4_f32_slice(
                self.view_loc.as_ref(),
                false,
                self.camera.view.as_slice(),
            );
            gl.uniform_matrix_4_f32_slice(
                self.projection_loc.as_ref(),
                false,
                self.projection_mat.as_slice(),
            );
        }
    }

    fn update_scene_data(&mut self, gl: &glow::Context, ux_data: &Controls) {
        self.directional_light_data.update_light(gl, self.program);
        for l in &mut self.point_light_data {
            l.update_light(gl, self.program);
        }

        for l in &mut self.spot_light_data {
            l.update_light(gl, self.program);
        }
        unsafe {
            gl.uniform_1_f32(
                self.time_loc.as_ref(),
                (self.time.elapsed().as_millis() as f32) * 0.01,
            );
            gl.uniform_1_i32(self.debug_loc.as_ref(), ux_data.debug);
            gl.uniform_1_i32(
                self.spot_light_number_loc.as_ref(),
                self.spot_light_data.len() as i32,
            );
            gl.uniform_1_i32(
                self.point_light_number_loc.as_ref(),
                self.point_light_data.len() as i32,
            );
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        rx: &Receiver<Frame>,
        need_refresh: bool,
        ux_data: &Controls,
        keyboard_data: &Vec<VirtualKeyCode>,
        mouse_position: (f64, f64),
    ) {
        // should handle the media here
        if !self.update_media && !need_refresh {
            return;
        }

        self.update_scene_data(gl, ux_data);
        self.update_matrix(gl, ux_data, keyboard_data, mouse_position);
        for m in &mut self.models {
            m.draw(gl, self.program, rx, &self.textures);
        }
    }

    pub fn init_gl_component(
        &mut self,
        gl: &glow::Context,
        config: &GraphicConfig,
        viewport_size: (f64, f64),
    ) {
        self.program = Self::create_program(gl, &config.vertex_path, &config.fragment_path);

        unsafe {
            gl.use_program(Some(self.program));
            for m in &mut self.models {
                m.init_gl_component(gl);
            }
        }
        self.camera = Camera::new(
            self.camera.position,
            self.camera.target,
            viewport_size,
            self.camera.last_mouse_position,
        );
        self.update_projection(viewport_size, self.fov);
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
