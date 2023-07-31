use super::point_light::PointLight;
use glow::*;
use iced_glow::glow;
use iced_glow::glow::Context;
use nalgebra_glm::Vec3;

pub struct SpotLight {
    light: PointLight,
    cut_off: f32,

    cut_off_loc: Option<NativeUniformLocation>,
}

impl SpotLight {
    pub fn new(gl: &Context, program: &NativeProgram, name: String, position: Vec3) -> Self {
        unsafe {
            Self {
                light: PointLight::new(gl, program, name.clone() + ".base", position),
                cut_off: 10.0_f32.to_radians().cos(),
                cut_off_loc: gl.get_uniform_location(*program, &(name + ".cut_off")),
            }
        }
    }

    pub fn update_light(&self, gl: &Context, program: NativeProgram) {
        self.light.update_light(gl, program);
        unsafe {
            gl.uniform_1_f32(self.cut_off_loc.as_ref(), self.cut_off);
        }
    }
}
