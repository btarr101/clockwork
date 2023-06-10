mod application;

use application::HelloWorld;
use clockwork::engine::Engine;

fn main() {
    Engine::run_application::<HelloWorld>();
}
