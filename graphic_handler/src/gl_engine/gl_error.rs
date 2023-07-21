use glow::*;
use iced_glow::glow;

pub fn gl_error(gl: &glow::Context, custom_error_msg: String) {
    let error;
    unsafe {
        error = gl.get_error();
    }
    if error != glow::NO_ERROR {
        match error {
            glow::INVALID_ENUM => panic!("[GL] Error: INVALID_ENUM | {}", custom_error_msg),
            glow::INVALID_VALUE => panic!("[GL] Error: INVALID_VALUE | {}", custom_error_msg),
            glow::INVALID_OPERATION => {
                panic!("[GL] Error: INVALID_OPERATION | {}", custom_error_msg)
            }
            glow::STACK_OVERFLOW => panic!("[GL] Error: STACK_OVERFLOW | {}", custom_error_msg),
            glow::STACK_UNDERFLOW => panic!("[GL] Error: STACK_UNDERFLOW | {}", custom_error_msg),
            glow::OUT_OF_MEMORY => panic!("[GL] Error: OUT_OF_MEMORY | {}", custom_error_msg),
            glow::INVALID_FRAMEBUFFER_OPERATION => {
                panic!(
                    "[GL] Error: INVALID_FRAMEBUFFER_OPERATION | {}",
                    custom_error_msg
                )
            }
            _ => panic!("[GL] Error: {} | {}", error, custom_error_msg),
        }
    }
}
