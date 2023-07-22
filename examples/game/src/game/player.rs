use std::{ time::Duration, cmp::{ min, max, Ordering } };

use clockwork::{
    input_state::InputState,
    input::Key,
    graphics_context::{ RenderOperation, QUAD_MESH },
    texture_atlas::{ SpriteId, TextureAtlas, LazySpriteId },
};
use glam::{ IVec2, Mat4 };
use num_traits::abs;
use super::{ tiles::Tiles, aabb::Aabb, constants::{ Unit, UnitVec2, UnitVec2Trait } };

pub struct Player {
    /// Maximum the speed the player can reach itself via running.
    pub max_run: Unit,

    /// How fast the player accelerates when running.
    pub run_accel: Unit,

    /// When the player stops running, how fast they deccelerate.
    pub run_deccel: Unit,

    /// How strong the players jump is
    pub jump_power: Unit,

    /// How long a jump input will be registered after is has been inputted.
    ///
    /// This is so if the player "jumps" right before they touch the ground, a
    /// jump is still triggered.
    pub jump_buffering: Duration,

    /// How many frames the player as before gravity starts applying.
    ///
    /// This is calculated in frames rather then in real time because it impacts
    /// game physics. The `cayote_timer` variable is updated every time `update()` is called,
    /// and when a player becomes grounded the timer is set to this `cayote_time`.
    pub cayote_time: u32,

    pub position: UnitVec2,
    pub velocity: UnitVec2,

    /// Timer for "cayote time", where the player can still jump for a period of time after walking
    /// off a ledge.
    pub cayote_timer: u32,

    /// Sprite used for the player
    sprite: LazySpriteId,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            max_run: 256,
            run_accel: 32,
            run_deccel: 16,
            jump_power: 450,
            jump_buffering: Duration::from_millis(100),
            cayote_time: 8,
            position: UnitVec2::ZERO,
            velocity: UnitVec2::ZERO,
            cayote_timer: 0,
            sprite: LazySpriteId::new("dummy32x32.png", Some("White")),
        }
    }
}

impl Player {
    /// Applies input to the player, as well as handles collisions with tiles.
    pub fn update(&mut self, tiles: &Tiles, input_state: &InputState) {
        let right = input_state.check_pressed(Key::D) as Unit;
        let left = input_state.check_pressed(Key::A) as Unit;
        let jump = input_state.check_pressed_within(Key::Space, self.jump_buffering);

        // Modify Velocity
        // =======================================================================
        // horizontal movement
        let horizontal_input = right - left;
        self.velocity.x += match horizontal_input {
            0 => {
                let damp = min(self.run_deccel, abs(self.velocity.x));
                -num_traits::sign::signum(self.velocity.x) * damp
            }
            _ => {
                let accel = horizontal_input * self.run_accel;
                if horizontal_input > 0 {
                    min(accel, self.max_run - self.velocity.x)
                } else {
                    max(accel, -self.max_run - self.velocity.x)
                }
            }
        };

        // jump or gravity
        if self.cayote_timer > 0 && jump {
            self.cayote_timer = 0;
            self.velocity.y = -self.jump_power;
        } else {
            self.velocity.y += 30;
        }

        // Apply Velocity and Fix
        // =======================================================================
        self.position.y += self.velocity.y;

        // Fix for vertical collision
        let mut aabb = Aabb {
            top_left: self.position,
            dimensions: IVec2 { x: 1000, y: 1000 },
        };
        if let Some(tile_aabb) = tiles.check_collision(&aabb) {
            self.position.y = match self.velocity.y.cmp(&0) {
                Ordering::Less => tile_aabb.y_bottom(),
                Ordering::Equal => panic!("Collision w/ no movement!"),
                Ordering::Greater => {
                    // side effect here a lil ugly
                    self.cayote_timer = self.cayote_time;
                    tile_aabb.y_top() - aabb.height()
                }
            };

            self.velocity.y = 0;
        } else {
            self.cayote_timer = self.cayote_timer.saturating_sub(1);
        }

        // apply horizontal velocity
        self.position.x += self.velocity.x;

        // fix for horizontal collisino
        aabb.top_left = self.position;
        if let Some(tile_aabb) = tiles.check_collision(&aabb) {
            self.position.x = match self.velocity.x.cmp(&0) {
                Ordering::Less => tile_aabb.x_right(),
                Ordering::Equal => panic!("Collision w/ no movement!"),
                Ordering::Greater => tile_aabb.x_left() - aabb.width(),
            };
            self.velocity.x = 0;
        }
    }

    pub fn get_render_operation(&self, atlas: &TextureAtlas, frame: usize) -> RenderOperation {
        let sprite = atlas.get_sprite_lazily(&self.sprite);
        let uv_window = sprite.get_uv_window(frame);

        RenderOperation {
            transform: Mat4::from_translation(
                self.position.to_render_vec2().extend(0.0)
            ).to_cols_array_2d(),
            uv_window,
            texture: sprite.texture,
            mesh: QUAD_MESH,
        }
    }
}
