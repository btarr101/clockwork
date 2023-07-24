//! # clockwork
//!
//! `clockwork` is a small game engine written in rust mainly for personal use.
//!

mod engine;

/// Keyboard input, mouse input, and etc.
pub mod input;
/// Rendering.
pub mod graphics;
/// Extra utility classes that aren't necessarily needed or may
/// be better if custom built. For example, [util::camera::Camera] is a class
/// that manages exporting a view projection matrix for rendering.
pub mod util;

pub use engine::{ Engine, Application, run };
