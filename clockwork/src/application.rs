use crate::engine::Engine;

pub trait Application: 'static {
    fn init(engine: &mut Engine) -> Self;
    fn update(&mut self, engine: &mut Engine, delta: f64);
}
