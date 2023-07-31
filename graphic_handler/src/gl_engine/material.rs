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
    specular_power: f32,
    specular_intensity: f32,

    ambient_loc: Option<NativeUniformLocation>,
    diffuse_loc: Option<NativeUniformLocation>,
    specular_loc: Option<NativeUniformLocation>,
    specular_power_loc: Option<NativeUniformLocation>,
    specular_intensity_loc: Option<NativeUniformLocation>,
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
        let ambient = Self::get_color(&raw_material.ambient);
        let diffuse = Self::get_color(&raw_material.diffuse);
        let specular = Self::get_color(&raw_material.specular);
        let specular_power = match raw_material.specular_exponent {
            Some(ne) => ne,
            _ => panic!("No Ns parameter"),
        };
        let specular_intensity = match raw_material.optical_density {
            Some(ni) => ni,
            _ => panic!("No Ns parameter"),
        };
        unsafe {
            Self {
                ambient,
                diffuse,
                specular,
                specular_power,
                specular_intensity,
                ambient_loc: gl.get_uniform_location(program, "material.ambient"),
                diffuse_loc: gl.get_uniform_location(program, "material.diffuse"),
                specular_loc: gl.get_uniform_location(program, "material.specular"),
                specular_power_loc: gl.get_uniform_location(program, "material.specular_power"),
                specular_intensity_loc: gl
                    .get_uniform_location(program, "material.specular_intensity"),
            }
        }
    }

    pub fn update_material(&self, gl: &Context, program: NativeProgram) {
        unsafe {
            gl.use_program(Some(program));
            gl.uniform_3_f32_slice(self.ambient_loc.as_ref(), self.ambient.as_slice());
            gl.uniform_3_f32_slice(self.diffuse_loc.as_ref(), self.diffuse.as_slice());
            gl.uniform_3_f32_slice(self.specular_loc.as_ref(), self.specular.as_slice());
            gl.uniform_1_f32(self.specular_power_loc.as_ref(), self.specular_power);
            gl.uniform_1_f32(
                self.specular_intensity_loc.as_ref(),
                self.specular_intensity,
            );
        }
    }
}
