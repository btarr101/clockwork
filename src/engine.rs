use crate::{
    graphics::RenderContext,
    input::InputState,
    input::{Keyboard, Mouse},
};

pub struct Engine {
    pub window: winit::window::Window,
    pub graphics_context: RenderContext,
    pub input_state: InputState,
}

pub trait Application: 'static {
    /// Called to create the application with the [Engine].
    fn init(engine: &mut Engine) -> Self;

    /// Called right before a frame renders.
    fn update(&mut self, engine: &mut Engine, delta: f64);

    /// Called whenever the application window is resized.
    #[allow(unused_variables)]
    fn on_window_resize(&mut self, engine: &mut Engine, new_size: glam::UVec2) {}
}

/// Instantiate an [Engine] that runs a Clockwork [Application].
pub fn run<App: Application>() {
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Clockwork Engine")
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let graphics_context = RenderContext::new(&window, size.width, size.height);

    let input_state = InputState::new();

    let mut engine = Engine {
        input_state,
        window,
        graphics_context,
    };

    let mut app = App::init(&mut engine);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state,
                        ..
                    },
                is_synthetic: false,
                ..
            } => {
                let key: Keyboard = num::FromPrimitive::from_u32(keycode as u32).unwrap();
                match state {
                    winit::event::ElementState::Pressed => engine.input_state.signal_press_of(key),
                    winit::event::ElementState::Released => {
                        engine.input_state.signal_release_of(key)
                    }
                }
            }
            winit::event::WindowEvent::MouseInput { button, state, .. } => {
                let button: Option<Mouse> = match button {
                    winit::event::MouseButton::Left => Some(Mouse::Left),
                    winit::event::MouseButton::Right => Some(Mouse::Right),
                    winit::event::MouseButton::Middle => Some(Mouse::Middle),
                    _ => None,
                };

                if let Some(button) = button {
                    match state {
                        winit::event::ElementState::Pressed => {
                            engine.input_state.signal_press_of(button)
                        }
                        winit::event::ElementState::Released => {
                            engine.input_state.signal_release_of(button)
                        }
                    }
                }
            }
            winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
            winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                let new_size = glam::UVec2 {
                    x: width,
                    y: height,
                };
                engine.graphics_context.resize_surface(new_size);
                app.on_window_resize(&mut engine, new_size);
            }
            _ => (),
        },
        winit::event::Event::MainEventsCleared => {
            app.update(&mut engine, 0.0);
        }
        _ => (),
    });
}
