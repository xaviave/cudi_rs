use glow::*;
use iced_glow::glow;
use iced_glow::glow::Context;
use nalgebra_glm::{vec3, Vec3};

pub struct BaseLight {
    color: Vec3,
    ambient_intensity: f32,
    diffuse_intensity: f32,

    color_loc: Option<NativeUniformLocation>,
    ambient_intensity_loc: Option<NativeUniformLocation>,
    diffuse_intensity_loc: Option<NativeUniformLocation>,
}

impl BaseLight {
    pub fn new(gl: &Context, program: &NativeProgram, name: String, diffuse: f32) -> Self {
        unsafe {
            Self {
                color: vec3(0.96, 1., 0.62),
                ambient_intensity: 0.05f32,
                diffuse_intensity: diffuse,

                color_loc: gl.get_uniform_location(*program, &(name.clone() + ".base.color")),
                ambient_intensity_loc: gl
                    .get_uniform_location(*program, &(name.clone() + ".base.ambient_intensity")),
                diffuse_intensity_loc: gl
                    .get_uniform_location(*program, &(name + ".base.diffuse_intensity")),
            }
        }
    }

    pub fn update_light(&self, gl: &Context, program: NativeProgram) {
        unsafe {
            gl.use_program(Some(program));
            gl.uniform_3_f32_slice(self.color_loc.as_ref(), self.color.as_slice());
            gl.uniform_1_f32(self.ambient_intensity_loc.as_ref(), self.ambient_intensity);
            gl.uniform_1_f32(self.diffuse_intensity_loc.as_ref(), self.diffuse_intensity);
        }
    }
}
