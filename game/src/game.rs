use std::f32::consts::PI;

use clockwork::{
    Application,
    util::camera::{ Camera, Projection },
    graphics::RenderOperation,
    Engine,
    util::texture_atlas::{ TextureAtlas, LazySpriteId },
};
use glam::{ Affine3A, IVec2, Vec3 };

mod constants;
mod tiles;
mod aabb;
mod player;

use tiles::Tiles;
use player::Player;

use constants::UnitVec2Trait;

use self::constants::{ PLAYER_TEXTURE, BLOCK_TEXTURE };

pub struct Game {
    camera: Camera,
    player: Player,
    tiles: Tiles,
    atlas: TextureAtlas,
    frame: f32,
}

impl Application for Game {
    fn init(engine: &mut Engine) -> Self {
        let mut atlas = TextureAtlas::new();
        let player_texture = engine.graphics_context.load_texture(PLAYER_TEXTURE).unwrap();
        let block_texture = engine.graphics_context.load_texture(BLOCK_TEXTURE).unwrap();
        atlas.add_aseprite_sprites(include_str!("../res/dummy32x32.json"), player_texture);
        atlas.add_aseprite_sprites(include_str!("../res/block.json"), block_texture);

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
                sprite: LazySpriteId::new("block.png", None),
            },
            atlas,
            frame: 0.0,
        }
    }

    fn update(&mut self, engine: &mut Engine, _delta: f64) {
        self.player.update(&self.tiles, &engine.input_state);

        let render_ops: Vec<RenderOperation> = self.tiles
            .get_render_operations(&self.atlas, self.frame as usize)
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

    fn on_window_resize(&mut self, _engine: &mut Engine, new_size: glam::UVec2) {
        let new_aspect = (new_size.x as f32) / (new_size.y as f32);
        match self.camera.mut_projection() {
            Projection::Perspective { aspect, .. } => {
                *aspect = new_aspect;
            }
        }
    }
}
