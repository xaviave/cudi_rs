use super::directional_light::DirectionalLight;
use glow::*;
use iced_glow::glow;
use iced_glow::glow::Context;
use nalgebra_glm::Vec3;

struct Attenuation {
    constant: f32,
    linear: f32,
    exp: f32,

    constant_loc: Option<NativeUniformLocation>,
    linear_loc: Option<NativeUniformLocation>,
    exp_loc: Option<NativeUniformLocation>,
}

impl Attenuation {
    pub fn new(gl: &Context, program: &NativeProgram, name: String, linear: f32) -> Self {
        unsafe {
            Self {
                constant: 0.3_f32,
                linear,
                exp: 3.0_f32,

                constant_loc: gl
                    .get_uniform_location(*program, &(name.clone() + ".attenuation.constant")),
                linear_loc: gl
                    .get_uniform_location(*program, &(name.clone() + ".attenuation.linear")),
                exp_loc: gl.get_uniform_location(*program, &(name + ".attenuation.exp_")),
            }
        }
    }

    pub fn update_attenuation(&self, gl: &Context, program: NativeProgram) {
        unsafe {
            gl.use_program(Some(program));
            gl.uniform_1_f32(self.constant_loc.as_ref(), self.constant);
            gl.uniform_1_f32(self.linear_loc.as_ref(), self.linear);
            gl.uniform_1_f32(self.exp_loc.as_ref(), self.exp);
        }
    }
}
pub struct PointLight {
    light: DirectionalLight,
    attenuation: Attenuation,
    position: Vec3,

    position_loc: Option<NativeUniformLocation>,
}

impl PointLight {
    pub fn new(gl: &Context, program: &NativeProgram, name: String, position: Vec3) -> Self {
        // to create raw point light: format!("point_lights[{}]", i);
        let base = DirectionalLight::new(gl, program, name.clone() + ".base", 0.9);
        Self {
            light: base,
            attenuation: Attenuation::new(gl, program, name.clone(), 0.1),
            position,
            position_loc: unsafe { gl.get_uniform_location(*program, &(name + ".position")) },
        }
    }

    pub fn update_light(&self, gl: &Context, program: NativeProgram) {
        self.light.update_light(gl, program);
        self.attenuation.update_attenuation(gl, program);

        unsafe {
            gl.use_program(Some(program));
            gl.uniform_3_f32_slice(self.position_loc.as_ref(), self.position.as_slice());
        }
    }
}
