use clockwork::{
    graphics::{ RenderOperation, CUBE_MESH },
    util::texture_atlas::{ TextureAtlas, LazySpriteId },
};
use glam::{ IVec2, Mat4 };

use super::{ constants::{ UnitVec2, UNITS_PER_TILE, UnitVec2Trait }, aabb::Aabb };

pub struct Tiles {
    pub position: UnitVec2,
    pub map: [[u32; Self::COLS as usize]; Self::ROWS as usize],
    pub sprite: LazySpriteId,
}

impl Tiles {
    const ROWS: i32 = 15;
    const COLS: i32 = 20;

    pub fn get_tile_coordinates(&self, position: UnitVec2) -> IVec2 {
        let relative_position = position - self.position;
        relative_position / UNITS_PER_TILE
    }

    /// Returns an [AABB] of the tile that was collided with if there was a collision, otherwise 'None'.
    pub fn check_collision(&self, other: &Aabb) -> Option<Aabb> {
        // Do a prelimnary check to see if a collision with this tilemap should even occur.
        let aabb = Aabb {
            top_left: self.position,
            dimensions: UnitVec2 {
                x: Self::COLS * UNITS_PER_TILE,
                y: Self::ROWS * UNITS_PER_TILE,
            },
        };

        if !aabb.collides_with(other) {
            return None;
        }

        // Do the tile collision check.
        let tile_top_left = self
            .get_tile_coordinates(other.top_left())
            .clamp(UnitVec2::ZERO, UnitVec2 { x: 19, y: 14 });
        let tile_bottom_right = self
            .get_tile_coordinates(other.bottom_right() + UnitVec2 { x: -1, y: -1 })
            .clamp(UnitVec2::ZERO, UnitVec2 { x: 19, y: 14 });

        for tile_x in tile_top_left.x..=tile_bottom_right.x {
            for tile_y in tile_top_left.y..=tile_bottom_right.y {
                // tile_x and tile_y should be gauranteed to be positive based on filter
                if self.map[tile_y as usize][tile_x as usize] != 0 {
                    let aabb = Aabb {
                        top_left: self.position +
                        UnitVec2 {
                            x: tile_x,
                            y: tile_y,
                        } *
                            UNITS_PER_TILE,
                        dimensions: UnitVec2::ONE * UNITS_PER_TILE,
                    };
                    return Some(aabb);
                }
            }
        }

        None
    }

    pub fn get_render_operations(
        &self,
        atlas: &TextureAtlas,
        frame: usize
    ) -> impl Iterator<Item = RenderOperation> + '_ {
        let sprite = *atlas.get_sprite_lazily(&self.sprite);

        let tiles = self.map
            .as_slice()
            .iter()
            .enumerate()
            .flat_map(|(tile_y, col)|
                col
                    .iter()
                    .enumerate()
                    .map(move |(tile_x, val)| (tile_x, tile_y, val))
            );

        tiles.filter_map(move |(tile_x, tile_y, val)| {
            if *val != 0 {
                let position =
                    self.position + IVec2 { x: tile_x as i32, y: tile_y as i32 } * UNITS_PER_TILE;
                Some(RenderOperation {
                    transform: Mat4::from_translation(
                        position.to_render_vec2().extend(0.0)
                    ).to_cols_array_2d(),
                    uv_window: sprite.get_uv_window(frame),
                    texture: sprite.texture,
                    mesh: CUBE_MESH,
                })
            } else {
                None
            }
        })
    }
}
