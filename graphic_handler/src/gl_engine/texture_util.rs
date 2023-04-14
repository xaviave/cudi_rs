use glow::*;
use iced_glow::glow;
use media_handler::frame::Frame;

pub trait TextureUtil {
    fn init_texture(gl: &glow::Context) -> NativeTexture {
        unsafe {
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            texture
        }
    }

    fn generate_texture(gl: &glow::Context, texture: NativeTexture, media: &Frame) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                RGBA as i32,
                media.width as i32,
                media.height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&media.get_raw_image()),
            );
            gl.generate_mipmap(glow::TEXTURE_2D);
        }
    }
}
