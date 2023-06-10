use crate::engine::Engine;

pub trait Application: 'static {
    fn init(engine: &Engine) -> Self;
    fn update(&mut self, engine: &Engine, delta: f64);
}
