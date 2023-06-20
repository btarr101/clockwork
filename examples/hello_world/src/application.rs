use glam::{ Affine3A, Mat4, Vec3, Vec3A };

use clockwork::{
    application::Application,
    engine::Engine,
    input::Key,
    graphics_context::{ MeshId, RenderOperation },
    mesh::Vertex,
    camera::Camera,
};

pub struct HelloWorld {
    camera: Camera,
    triangle_affine: Affine3A,
    triangle_mesh: MeshId,
}

impl Application for HelloWorld {
    fn init(engine: &mut Engine) -> Self {
        Self {
            camera: Camera::default(),
            triangle_affine: Affine3A::from_translation(Vec3 { x: 0.0, y: 0.0, z: -0.5 }),
            triangle_mesh: engine.graphics_context.load_mesh(
                &[
                    Vertex {
                        position: [-0.5, 0.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                        texture_coordinates: [0.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, 0.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                        texture_coordinates: [0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 1.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                        texture_coordinates: [0.0, 0.0],
                    },
                ],
                &[0, 1, 2]
            ),
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
            0.001;
        self.camera.affine.translation += Vec3A::from(movement);

        engine.graphics_context.perform_render_pass(
            self.camera.get_view_projection_matrix().to_cols_array_2d(),
            &[
                RenderOperation {
                    transform: Mat4::from(self.triangle_affine).to_cols_array_2d(),
                    mesh: self.triangle_mesh,
                },
            ]
        )
    }
}
