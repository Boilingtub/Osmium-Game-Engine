#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}


pub struct CameraController {
    speed: f32,
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

use winit::event::{WindowEvent, KeyEvent, ElementState};
use winit::keyboard::{PhysicalKey,KeyCode};

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
            ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.forward_pressed = is_pressed;
                        true
                    },
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}


pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub uniform: CameraUniform,
    pub controller: CameraController,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect,
                                       self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn update_view_proj(&mut self) {
        self.uniform.view_proj = self.build_view_projection_matrix().into();
    }

    pub fn update_controller(&mut self) {
        use cgmath::InnerSpace;
        let forward = self.target - self.eye;
        let forward_norm = forward.normalize();
        let formard_mag = forward.magnitude();

        if self.controller.forward_pressed && formard_mag > self.controller.speed {
            self.eye += forward_norm * self.controller.speed;
        }
        if self.controller.backward_pressed {
            self.eye -= forward_norm * self.controller.speed;
        } 

        let right = forward_norm.cross(self.up);
        let forward = self.target - self.eye;
        let forward_mag = forward.magnitude();

        if self.controller.right_pressed {
            self.eye = self.target - 
                         (forward + right * self.controller.speed).normalize() * forward_mag;
        }
        if self.controller.left_pressed {
            self.eye = self.target -
                         (forward - right * self.controller.speed).normalize() * forward_mag;
        }
    }
}
