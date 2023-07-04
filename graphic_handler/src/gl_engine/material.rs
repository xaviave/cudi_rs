use glow::*;
use iced_glow::glow;
use nalgebra_glm::vec3;
use nalgebra_glm::Vec3;

pub struct Material {
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    shininess: f32,

    ambient_loc: Option<NativeUniformLocation>,
    diffuse_loc: Option<NativeUniformLocation>,
    specular_loc: Option<NativeUniformLocation>,
    shininess_loc: Option<NativeUniformLocation>,
}

impl Material {
    pub fn new(gl: &Context, program: NativeProgram) -> Self {
        unsafe {
            Self {
                ambient: vec3(1., 0.5, 0.31),
                diffuse: vec3(1., 0.5, 0.31),
                specular: vec3(0.5, 0.5, 0.5),
                shininess: 32.0,
                ambient_loc: gl.get_uniform_location(program, "material.ambient"),
                diffuse_loc: gl.get_uniform_location(program, "material.diffuse"),
                specular_loc: gl.get_uniform_location(program, "material.specular"),
                shininess_loc: gl.get_uniform_location(program, "material.shininess"),
            }
        }
    }

    pub fn update_material(&self, gl: &Context, program: NativeProgram) {
        unsafe {
            gl.use_program(Some(program));
            gl.uniform_3_f32_slice(self.ambient_loc.as_ref(), self.ambient.as_slice());
            gl.uniform_3_f32_slice(self.diffuse_loc.as_ref(), self.diffuse.as_slice());
            gl.uniform_3_f32_slice(self.specular_loc.as_ref(), self.specular.as_slice());
            gl.uniform_1_f32(self.shininess_loc.as_ref(), self.shininess);
        }
    }
}
