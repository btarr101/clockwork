mod game;
pub use game::Game;

use clockwork::engine::Engine;

fn main() {
    Engine::run_application::<Game>();
}
