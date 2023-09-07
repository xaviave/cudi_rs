use iced_winit::winit::event::VirtualKeyCode;
use nalgebra_glm::{look_at, vec3, Mat4, Vec3};

pub struct Camera {
    step_size: f32,
    sensitivity: f64,
    pub viewport_size: (f64, f64),
    // position
    pub position: Vec3,
    pub target: Vec3,
    up: Vec3,
    // target
    // flag enable if mouse out of screen to allow camera movement
    out_screen: f64,
    yaw: f64,
    pitch: f64,
    pub last_mouse_position: (f64, f64),

    pub view: Mat4,
}

impl Camera {
    fn update_view_matrix(position: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        look_at(&position, &(position + target), &up)
    }

    pub fn new(
        position: Vec3,
        target: Vec3,
        viewport_size: (f64, f64),
        mouse_position: (f64, f64),
    ) -> Self {
        let up = vec3(0.0, 1.0, 0.0);
        Self {
            step_size: 0.1,
            sensitivity: 0.3,
            viewport_size,

            position,
            target: target.normalize(),
            up,

            out_screen: 0.,
            yaw: 90.,
            pitch: -30.,
            last_mouse_position: mouse_position,

            view: Self::update_view_matrix(position, target, up),
        }
    }

    fn update_target(&mut self, mut mouse_position: (f64, f64)) {
        mouse_position = (250., 250.);

        self.out_screen = if mouse_position.0 >= self.viewport_size.1 - 20. {
            1.
        } else if mouse_position.0 <= 20. {
            -1.
        } else {
            0.
        };

        self.yaw += if self.out_screen == 0. {
            (mouse_position.0 - self.last_mouse_position.0) * self.sensitivity
        } else {
            5. * self.sensitivity * self.out_screen
        };

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
    }

    fn update_position(&mut self, keyboard_data: &Vec<VirtualKeyCode>) {
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
    }

    pub fn update_camera(
        &mut self,
        keyboard_data: &Vec<VirtualKeyCode>,
        mouse_position: (f64, f64),
    ) {
        self.update_target(mouse_position);
        self.update_position(keyboard_data);
        self.view = Self::update_view_matrix(self.position, self.target, self.up);
    }
}
