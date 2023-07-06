use glow::*;
use iced_glow::glow;
use nalgebra_glm::vec3;
use nalgebra_glm::Vec3;
use obj::raw::material::Material;
use obj::raw::material::MtlColor;
use obj::raw::material::MtlColor::Rgb;

pub struct CMaterial {
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    shininess: f32,

    ambient_loc: Option<NativeUniformLocation>,
    diffuse_loc: Option<NativeUniformLocation>,
    specular_loc: Option<NativeUniformLocation>,
    shininess_loc: Option<NativeUniformLocation>,
}

impl CMaterial {
    fn get_color(c: &Option<MtlColor>) -> Vec3 {
        match c {
            Some(color) => match color {
                Rgb(r, g, b) => vec3(*r, *g, *b),
                _ => panic!("Not rgb color"),
            },
            _ => vec3(0., 0., 0.),
        }
    }

    pub fn new(gl: &Context, program: NativeProgram, raw_material: &Material) -> Self {
        println!("{:?}", raw_material);
        let ambient = Self::get_color(&raw_material.ambient);
        let diffuse = Self::get_color(&raw_material.diffuse);
        let specular = Self::get_color(&raw_material.specular);
        let shininess = match raw_material.specular_exponent {
            Some(ns) => ns,
            _ => panic!("No Ns parameter"),
        };
        unsafe {
            Self {
                ambient,
                diffuse,
                specular,
                shininess,
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
