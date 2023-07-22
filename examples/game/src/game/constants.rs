use clockwork::texture_atlas::SpriteId;
use glam::{ Vec2, IVec2 };

/// Type for the smallest unit in the game.
pub type Unit = i32;

/// Type for a vec2 of units.
pub type UnitVec2 = IVec2;

/// How many in game units represent a single tile.
pub const UNITS_PER_RENDER_UNIT: Unit = 1000;
pub const UNITS_PER_TILE: Unit = 1000;

// TEMP
pub const TEXTURE_BYTES: &[u8] = include_bytes!("../../res/dummy32x32.png");
pub const DUMMY: &str = "dummy32x32.png";

/// Trait for the smalles unit in the game.
pub trait UnitTrait {
    /// Converts [Unit]'s - the smallest in game increment - to Tile units.
    fn to_tile_units(self) -> Self;

    /// Converts [Units]'s to render units.
    fn to_render_units(self) -> f32;
}

/// Trait for a vector of units.
pub trait UnitVec2Trait {
    /// Converts this structure into a [Vec2] that represents render units.
    fn to_render_vec2(self) -> Vec2;
}

impl UnitTrait for i32 {
    fn to_tile_units(self) -> Self {
        self / UNITS_PER_TILE
    }

    fn to_render_units(self) -> f32 {
        (self as f32) / (UNITS_PER_RENDER_UNIT as f32)
    }
}

impl UnitVec2Trait for IVec2 {
    fn to_render_vec2(self) -> Vec2 {
        Vec2 {
            x: self.x.to_render_units(),
            y: -self.y.to_render_units(),
        }
    }
}
