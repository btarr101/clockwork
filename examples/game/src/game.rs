use std::f32::consts::PI;

use clockwork::{
    application::Application,
    camera::{ Camera, Projection },
    graphics_context::RenderOperation,
    engine::Engine,
    texture_atlas::{ TextureAtlas, LazySpriteId },
};
use glam::{ Affine3A, IVec2, Vec3 };

mod constants;
mod tiles;
mod aabb;
mod player;

use tiles::Tiles;
use player::Player;

use constants::UnitVec2Trait;

use self::constants::TEXTURE_BYTES;
pub struct Game {
    camera: Camera,
    player: Player,
    tiles: Tiles,
    atlas: TextureAtlas,
    frame: f32,
}

impl Application for Game {
    fn init(engine: &mut clockwork::engine::Engine) -> Self {
        let mut atlas = TextureAtlas::new();
        let texture = engine.graphics_context.load_texture(TEXTURE_BYTES).unwrap();
        atlas.add_aseprite_sprites(include_str!("../res/dummy32x32.json"), texture);

        Self {
            camera: Camera::new(
                Affine3A::from_translation(Vec3 { x: 0.0, y: 0.0, z: 10.0 }),
                Projection::Perspective {
                    aspect: {
                        let size = engine.window.inner_size();
                        (size.width as f32) / (size.height as f32)
                    },
                    fov: PI / 2.0,
                    znear: 0.1,
                    zfar: 100.0,
                }
            ),
            player: Default::default(),
            tiles: Tiles {
                position: IVec2 { x: 1000 * -8, y: 1000 * -5 },
                map: [
                    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1],
                    [1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                ],
                sprite: LazySpriteId::new("dummy32x32.png", Some("Black")),
            },
            atlas,
            frame: 0.0,
        }
    }

    fn update(&mut self, engine: &mut Engine, _delta: f64) {
        self.player.update(&self.tiles, &engine.input_state);

        let render_ops: Vec<RenderOperation> = self.tiles
            .get_render_operations(&self.atlas)
            .chain(
                std::iter::once(self.player.get_render_operation(&self.atlas, self.frame as usize))
            )
            .collect();

        let camera_translation_target = self.player.position
            .to_render_vec2()
            .extend(8.0 + 1.0 * self.player.velocity.to_render_vec2().length());

        self.camera.affine.translation = self.camera.affine.translation.lerp(
            camera_translation_target.into(),
            0.2
        );

        engine.graphics_context.perform_render_pass(
            self.camera.get_view_projection_matrix().to_cols_array_2d(),
            render_ops.as_slice()
        );

        self.frame += 0.1;
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
