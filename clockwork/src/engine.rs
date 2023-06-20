use winit::{
    window::{ Window, WindowBuilder },
    event_loop::EventLoop,
    event::{ Event, WindowEvent, KeyboardInput, ElementState, self },
    dpi::PhysicalSize,
};

use crate::{
    graphics_context::GraphicsContext,
    application::Application,
    input_state::InputState,
    input::{ Key, MouseButton },
};

pub struct Engine {
    pub window: Window,
    pub graphics_context: GraphicsContext,
    pub input_state: InputState,
}

impl Engine {
    pub fn run_application<App: Application>() {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Clockwork Engine")
            .build(&event_loop)
            .unwrap();

        let size = window.inner_size();
        let graphics_context = GraphicsContext::new(&window, size.width, size.height);

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
                            let key: Key = num::FromPrimitive::from_u32(keycode as u32).unwrap();
                            match state {
                                ElementState::Pressed => engine.input_state.signal_press_of(key),
                                ElementState::Released => engine.input_state.signal_release_of(key),
                            }
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            let button: Option<MouseButton> = match button {
                                event::MouseButton::Left => Some(MouseButton::Left),
                                event::MouseButton::Right => Some(MouseButton::Right),
                                event::MouseButton::Middle => Some(MouseButton::Middle),
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
                        WindowEvent::Resized(PhysicalSize { width, height }) =>
                            engine.graphics_context.resize_surface(width, height),
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
