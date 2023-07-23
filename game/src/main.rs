mod game;
pub use game::Game;

fn main() {
    clockwork::Engine::run_application::<Game>();
}
