use winit::{
    window::{ Window, WindowBuilder },
    event_loop::EventLoop,
    event::{ Event, WindowEvent, KeyboardInput, ElementState, self },
    dpi::PhysicalSize,
};

use crate::{ graphics::Context, input::InputState, input::{ Keyboard, Mouse } };

pub struct Engine {
    pub window: Window,
    pub graphics_context: Context,
    pub input_state: InputState,
}

pub trait Application: 'static {
    /// Called when the application is initialized.
    fn init(engine: &mut Engine) -> Self;

    /// Called right before a frame renders.
    fn update(&mut self, engine: &mut Engine, delta: f64);

    // callbacks
    /// Called whenever the application window is resized.
    #[allow(unused_variables)]
    fn on_window_resize(&mut self, engine: &mut Engine, new_size: glam::UVec2) {}
}

impl Engine {
    pub fn run_application<App: Application>() {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Clockwork Engine")
            .build(&event_loop)
            .unwrap();

        let size = window.inner_size();
        let graphics_context = Context::new(&window, size.width, size.height);

        let input_state = InputState::new();

        let mut engine = Engine {
            input_state,
            window,
            graphics_context,
        };

        let mut app = App::init(&mut engine);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } =>
                    match event {
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput { virtual_keycode: Some(keycode), state, .. },
                            is_synthetic: false,
                            ..
                        } => {
                            let key: Keyboard = num::FromPrimitive
                                ::from_u32(keycode as u32)
                                .unwrap();
                            match state {
                                ElementState::Pressed => engine.input_state.signal_press_of(key),
                                ElementState::Released => engine.input_state.signal_release_of(key),
                            }
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            let button: Option<Mouse> = match button {
                                event::MouseButton::Left => Some(Mouse::Left),
                                event::MouseButton::Right => Some(Mouse::Right),
                                event::MouseButton::Middle => Some(Mouse::Middle),
                                _ => None,
                            };

                            if let Some(button) = button {
                                match state {
                                    ElementState::Pressed =>
                                        engine.input_state.signal_press_of(button),
                                    ElementState::Released =>
                                        engine.input_state.signal_release_of(button),
                                }
                            }
                        }
                        WindowEvent::CloseRequested => control_flow.set_exit(),
                        WindowEvent::Resized(PhysicalSize { width, height }) => {
                            let new_size = glam::UVec2 { x: width, y: height };
                            engine.graphics_context.resize_surface(new_size);
                            app.on_window_resize(&mut engine, new_size);
                        }
                        _ => (),
                    }
                Event::MainEventsCleared => {
                    app.update(&mut engine, 0.0);
                }
                _ => (),
            }
        });
    }
}
