use glow::*;
use iced_glow::glow;

use crate::gl_program::GlProgram;

use nalgebra_glm::{perspective, rotation, translation, vec3, TMat4, TVec3};

pub struct Scene {
    pub ratio: f32,
    pub last_pos: TVec3<f32>,

    model: TMat4<f32>,
    view: TMat4<f32>,
    projection: TMat4<f32>,

    model_loc: Option<NativeUniformLocation>,
    view_loc: Option<NativeUniformLocation>,
    projection_loc: Option<NativeUniformLocation>,
}

impl Scene {
    pub fn new(gl: &Context, program: &NativeProgram) -> Self {
        unsafe {
            Self {
                ratio: 1.,
                last_pos: vec3(0., 0., 0.),
                model: rotation(0.0_f32.to_radians(), &(vec3(0.5, 1.0, 0.0).normalize())),
                view: translation(&(vec3(0., 0., -3.).normalize())),
                projection: perspective(1., (45_f32).to_radians(), 0.1, 100.0),
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
        unsafe {
            self.update_model(gl, self.model);
            gl.uniform_matrix_4_f32_slice(self.view_loc.as_ref(), false, self.view.as_slice());
            gl.uniform_matrix_4_f32_slice(
                self.projection_loc.as_ref(),
                false,
                self.projection.as_slice(),
            );
        }
    }
}
