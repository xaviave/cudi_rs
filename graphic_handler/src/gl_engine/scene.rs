use glow::*;
use iced_glow::glow;

use crate::gl_engine::{light::Light, material::Material};

use nalgebra_glm::{perspective, rotation, translation, vec3, TMat4, TVec3};

pub struct Scene {
    pub texture_ratio: f32,
    pub viewport_ratio: f32,
    pub last_pos: TVec3<f32>,

    raw_vertex_buffer: Vec<f32>,
    pub raw_indices_buffer: Vec<i32>,
    pub light_data: Light,
    pub material_data: Material,
    pub model: TMat4<f32>,

    view: TMat4<f32>,
    projection: TMat4<f32>,

    model_loc: Option<NativeUniformLocation>,
    view_loc: Option<NativeUniformLocation>,
    projection_loc: Option<NativeUniformLocation>,
}

impl Scene {
    pub fn new(
        gl: &Context,
        program: &NativeProgram,
        viewport_ratio: f32,
        raw_vertex_buffer: Vec<f32>,
        raw_indices_buffer: Vec<i32>,
    ) -> Self {
        unsafe {
            Self {
                texture_ratio: 1.,
                viewport_ratio,
                last_pos: vec3(0., 0., 0.),
                raw_vertex_buffer,
                raw_indices_buffer,
                light_data: Light::new(gl, program),
                material_data: Material::new(gl, program),
                model: translation(&(vec3(0., -1., -5.)))
                    * rotation(90.0_f32.to_radians(), &(vec3(0., 1.0, 0.0).normalize())),
                view: translation(&(vec3(0., 0., -3.).normalize())),
                projection: perspective(viewport_ratio, (45_f32).to_radians(), 0.1, 100.0),
                model_loc: gl.get_uniform_location(*program, "model"),
                view_loc: gl.get_uniform_location(*program, "view"),
                projection_loc: gl.get_uniform_location(*program, "projection"),
            }
        }
    }

    pub fn update_model(&self, gl: &Context, model: TMat4<f32>) {
        unsafe {
            gl.uniform_matrix_4_f32_slice(self.model_loc.as_ref(), false, model.as_slice());
        }
    }

    pub fn update_scene(&self, gl: &Context) {
        self.light_data.update_light(gl);
        self.material_data.update_material(gl);
        unsafe {
            gl.uniform_matrix_4_f32_slice(self.view_loc.as_ref(), false, self.view.as_slice());
            gl.uniform_matrix_4_f32_slice(
                self.projection_loc.as_ref(),
                false,
                self.projection.as_slice(),
            );
        }
    }
}
