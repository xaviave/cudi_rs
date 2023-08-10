use iced_winit::winit::event::VirtualKeyCode;
use nalgebra_glm::{look_at, vec3, Mat4, Vec3};

pub struct Camera {
    step_size: f32,
    sensitivity: f64,
    // position
    pub position: Vec3,
    pub target: Vec3,
    up: Vec3,
    // target
    yaw: f64,
    pitch: f64,
    pub last_mouse_position: (f64, f64),

    pub view: Mat4,
}

impl Camera {
    fn update_view_matrix(position: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        look_at(&position, &(position + target), &up)
    }

    pub fn new(position: Vec3, target: Vec3, mouse_position: (f64, f64)) -> Self {
        let up = vec3(0.0, 1.0, 0.0);
        Self {
            step_size: 0.1,
            sensitivity: 0.3,

            position,
            target: target.normalize(),
            up,

            yaw: -90.,
            pitch: 0.,
            last_mouse_position: mouse_position,

            view: Self::update_view_matrix(position, target, up),
        }
    }

    pub fn update_camera(
        &mut self,
        keyboard_data: &Vec<VirtualKeyCode>,
        mouse_position: (f64, f64),
    ) {
        self.yaw += (mouse_position.0 - self.last_mouse_position.0) * self.sensitivity;
        self.pitch += (self.last_mouse_position.1 - mouse_position.1) * self.sensitivity;
        self.last_mouse_position = (mouse_position.0, mouse_position.1);

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        } else if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.target = vec3(
            (self.yaw.to_radians().cos() * self.pitch.to_radians().cos()) as f32,
            (self.pitch.to_radians().sin()) as f32,
            (self.yaw.to_radians().sin() * self.pitch.to_radians().cos()) as f32,
        )
        .normalize();

        for k in keyboard_data {
            match k {
                VirtualKeyCode::W => self.position += self.target * self.step_size,
                VirtualKeyCode::S => self.position -= self.target * self.step_size,
                VirtualKeyCode::A => {
                    let l = self.target.cross(&self.up).normalize();
                    self.position -= l * self.step_size
                }
                VirtualKeyCode::D => {
                    let r = self.target.cross(&self.up).normalize();
                    self.position += r * self.step_size
                }
                VirtualKeyCode::PageUp => {
                    self.position += vec3(0.0, self.step_size, 0.0);
                }
                VirtualKeyCode::PageDown => {
                    self.position -= vec3(0.0, self.step_size, 0.0);
                }
                _ => (),
            }
        }
        //             fov -= (float)yoffset;
        // if (fov < 1.0f)
        //     fov = 1.0f;
        // if (fov > 45.0f)
        //     fov = 45.0f;

        self.view = Self::update_view_matrix(self.position, self.target, self.up);
    }
}
