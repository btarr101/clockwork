mod game;
pub use game::Game;

use clockwork::{ engine::Engine, texture_atlas::TextureAtlas };

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    Engine::run_application::<Game>();
}
