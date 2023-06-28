use std::f32::consts::PI;

use glam::{ Affine3A, Mat4, Vec3, Vec3A };

use clockwork::{
    application::Application,
    engine::Engine,
    input::Key,
    graphics_context::{ RenderOperation, QUAD_MESH },
    camera::{ Camera, Projection },
};

pub struct HelloWorld {
    camera: Camera,
    affine: Affine3A,
}

impl Application for HelloWorld {
    fn init(engine: &mut Engine) -> Self {
        Self {
            camera: Camera::new(Affine3A::IDENTITY, Projection::Perspective {
                aspect: {
                    let size = engine.window.inner_size();
                    (size.width as f32) / (size.height as f32)
                },
                fov: PI / 2.0,
                znear: 0.01,
                zfar: 100.0,
            }),
            affine: Affine3A::from_translation(Vec3 { x: 0.0, y: 0.0, z: -0.5 }),
        }
    }

    fn update(&mut self, engine: &mut Engine, _delta: f64) {
        let right = engine.input_state.check_pressed(Key::D) as i32 as f32;
        let left = engine.input_state.check_pressed(Key::A) as i32 as f32;

        let forward = engine.input_state.check_pressed(Key::W) as i32 as f32;
        let back = engine.input_state.check_pressed(Key::S) as i32 as f32;

        let up = engine.input_state.check_pressed(Key::Space) as i32 as f32;
        let down = engine.input_state.check_pressed(Key::LShift) as i32 as f32;

        let movement =
            ((Vec3 { x: right - left, y: 0.0, z: back - forward }).normalize_or_zero() +
                Vec3 { x: 0.0, y: up - down, z: 0.0 }) *
            0.1;
        self.camera.affine.translation += Vec3A::from(movement);

        // TODO: OPTION FOR NO TEXTURE JUST COLOR! - like texture or color
        // engine.graphics_context.perform_render_pass(
        //     self.camera.get_view_projection_matrix().to_cols_array_2d(),
        //     &[
        //         RenderOperation {
        //             transform: Mat4::from(self.affine).to_cols_array_2d(),
        //             mesh: QUAD_MESH,
        //         },
        //     ]
        // )
    }

    fn on_window_resize(&mut self, _engine: &mut Engine, new_width: u32, new_height: u32) {
        let new_aspect = (new_width as f32) / (new_height as f32);
        match self.camera.mut_projection() {
            Projection::Perspective { aspect, .. } => {
                *aspect = new_aspect;
            }
        }
    }
}
