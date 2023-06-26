use crate::engine::Engine;

pub trait Application: 'static {
    fn init(engine: &mut Engine) -> Self;
    fn update(&mut self, engine: &mut Engine, delta: f64);

    // callbacks
    #[allow(unused_variables)]
    fn on_window_resize(&mut self, engine: &mut Engine, new_width: u32, new_height: u32) {}
}
