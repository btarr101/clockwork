use std::{ time::Duration, cmp::{ min, max, Ordering } };

use clockwork::{
    input_state::InputState,
    input::Key,
    graphics_context::{ RenderOperation, QUAD_MESH },
};
use glam::{ IVec2, Mat4 };
use num_traits::abs;
use super::{ tiles::Tiles, MAX_RUN, aabb::Aabb, constants::{ Unit, UnitVec2, UnitVec2Trait } };

pub struct Player {
    /// Maximum the speed the player can reach itself via running.
    pub max_run: Unit,

    /// How fast the player accelerates when running.
    pub run_accel: Unit,

    /// When the player stops running, how fast they deccelerate.
    pub run_deccel: Unit,

    /// How long a jump input will be registered after is has been inputted.
    ///
    /// This is so if the player "jumps" right before they touch the ground, a
    /// jump is still triggered.
    pub jump_buffering: Duration,

    pub position: UnitVec2,
    pub velocity: UnitVec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            max_run: 250,
            run_accel: 32,
            run_deccel: 16,
            jump_buffering: Duration::from_millis(200),
            position: UnitVec2::ZERO,
            velocity: UnitVec2::ZERO,
        }
    }
}

impl Player {
    /// Applies input to the player, as well as handles collisions with tiles.
    pub fn update(&mut self, tiles: &Tiles, input_state: &InputState) {
        let right = input_state.check_pressed(Key::D) as Unit;
        let left = input_state.check_pressed(Key::A) as Unit;
        let jump = input_state.check_pressed_within(Key::Space, self.jump_buffering);

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
                    min(accel, MAX_RUN - self.velocity.x)
                } else {
                    max(accel, -MAX_RUN - self.velocity.x)
                }
            }
        };

        // gravity
        self.velocity.y += 30;

        // apply velocity
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
                Ordering::Greater => tile_aabb.y_top() - aabb.height(),
            };

            self.velocity.y = if self.velocity.y > 0 && jump { -500 } else { 0 };
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

    pub fn get_render_operation(&self) -> RenderOperation {
        RenderOperation {
            transform: Mat4::from_translation(
                self.position.to_render_vec2().extend(0.0)
            ).to_cols_array_2d(),
            mesh: QUAD_MESH,
        }
    }
}
