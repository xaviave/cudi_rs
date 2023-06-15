use glow::*;
use iced_glow::glow;
use nalgebra_glm::vec3;
use nalgebra_glm::Vec3;

pub struct Light {
    position: Vec3,
    direction: Vec3,

    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,

    position_loc: Option<NativeUniformLocation>,
    direction_loc: Option<NativeUniformLocation>,
    ambient_loc: Option<NativeUniformLocation>,
    diffuse_loc: Option<NativeUniformLocation>,
    specular_loc: Option<NativeUniformLocation>,
}

impl Light {
    pub fn new(gl: &Context, program: &NativeProgram) -> Self {
        unsafe {
            Self {
                position: vec3(0., 0., -3.),
                direction: vec3(0., 1., 0.),

                ambient: vec3(0.01, 0.01, 0.01),
                diffuse: vec3(0.1, 0.1, 0.1),
                specular: vec3(1., 1., 0.),

                position_loc: gl.get_uniform_location(*program, "light.position"),
                direction_loc: gl.get_uniform_location(*program, "light.direction"),
                ambient_loc: gl.get_uniform_location(*program, "light.ambient"),
                diffuse_loc: gl.get_uniform_location(*program, "light.diffuse"),
                specular_loc: gl.get_uniform_location(*program, "light.specular"),
            }
        }
    }

    pub fn update_light(&self, gl: &Context) {
        unsafe {
            gl.uniform_3_f32_slice(self.position_loc.as_ref(), self.position.as_slice());
            gl.uniform_3_f32_slice(self.direction_loc.as_ref(), self.direction.as_slice());
            gl.uniform_3_f32_slice(self.ambient_loc.as_ref(), self.ambient.as_slice());
            gl.uniform_3_f32_slice(self.diffuse_loc.as_ref(), self.diffuse.as_slice());
            gl.uniform_3_f32_slice(self.specular_loc.as_ref(), self.specular.as_slice());
        }
    }
}
