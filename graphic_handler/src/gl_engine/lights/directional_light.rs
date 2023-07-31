use super::basic_light::BaseLight;
use glow::*;
use iced_glow::glow;
use iced_glow::glow::Context;
use nalgebra_glm::{vec3, Vec3};

pub struct DirectionalLight {
    light: BaseLight,
    direction: Vec3,

    direction_loc: Option<NativeUniformLocation>,
}

impl DirectionalLight {
    pub fn new(gl: &Context, program: &NativeProgram, name: String, diffuse: f32) -> Self {
        // to create raw directional light: name should be name = String::from("directional_light")
        let base = BaseLight::new(gl, program, name.clone(), diffuse);
        unsafe {
            Self {
                light: base,
                direction: vec3(1.0, -1.0, 0.0),
                direction_loc: gl.get_uniform_location(*program, &(name + ".direction")),
            }
        }
    }

    pub fn update_light(&self, gl: &Context, program: NativeProgram) {
        self.light.update_light(gl, program);
        unsafe {
            gl.uniform_3_f32_slice(self.direction_loc.as_ref(), self.direction.as_slice());
        }
    }
}
