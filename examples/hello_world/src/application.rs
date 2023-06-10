use std::time::Duration;

use clockwork::{ application::Application, engine::Engine, input::Key };

pub struct HelloWorld {}

impl Application for HelloWorld {
    fn init(_engine: &Engine) -> Self {
        Self {}
    }

    fn update(&mut self, engine: &Engine, _delta: f64) {
        if engine.input_state.check_pressed_within(Key::A, Duration::from_millis(1)) {
            println!("A pressed within 100 millis!");
        }
    }
}
