use std::{ time::{ Instant, Duration } };

use crate::input::{ MAX_MOUSEBUTTON, MAX_KEY, Key, MouseButton };

const INPUTS: usize = MAX_MOUSEBUTTON + MAX_KEY;

// Note, very importantly with this current implementation, I'm sad to say this
// will only work for 584942417355.072 years.
pub enum Input {
    Keyboard(Key),
    MouseButton(MouseButton),
}

/// Manages storring the current state of an application user's inputs.
pub struct InputState {
    // important to note that a state is only valid if it has a
    // non-None timestamp.
    states: [bool; INPUTS],
    press_timestamps: [Option<Instant>; INPUTS],
    releaste_timestamps: [Option<Instant>; INPUTS],
}

impl From<Key> for Input {
    fn from(val: Key) -> Self {
        Input::Keyboard(val)
    }
}

impl From<MouseButton> for Input {
    fn from(val: MouseButton) -> Self {
        Input::MouseButton(val)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            states: [false; INPUTS],
            press_timestamps: [None; INPUTS],
            releaste_timestamps: [None; INPUTS],
        }
    }
}

impl InputState {
    /// Creates a fresh [InputState].
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if an [Input] is currently pressed.
    pub fn check_pressed<I: Into<Input>>(&self, input: I) -> bool {
        self.states[Self::get_state_index(input.into())]
    }

    /// Checks if an [Input] is currently released.
    pub fn check_released<I: Into<Input>>(&self, input: I) -> bool {
        !self.states[Self::get_state_index(input.into())]
    }

    /// Checks if an [Input] was pressed within a [Duration].
    ///
    /// This is useful for checking if an [Input] was just pressed rather than if it is held
    /// down.
    pub fn check_pressed_within<I: Into<Input>>(&self, input: I, duration: Duration) -> bool {
        self.check_within_duration(input.into(), duration, true)
    }

    /// Checks if an [Input] was released within a [Duration].
    ///
    /// This is useful for checking if an [Input] was just released rather than if it isn't
    /// held down.
    pub fn check_released_within<I: Into<Input>>(&self, input: I, duration: Duration) -> bool {
        self.check_within_duration(input.into(), duration, false)
    }

    /// Signals to the [InputState] that a specific [Input] was pressed.
    ///
    /// Will ignore if the [Input] is already pressed.
    pub fn signal_press_of<I: Into<Input>>(&mut self, input: I) {
        let index = Self::get_state_index(input.into());
        if !self.states[index] {
            self.states[index] = true;
            self.press_timestamps[index] = Some(Instant::now());
        }
    }

    /// Signals to the [InputState] that a specific [Input] was released.
    pub fn signal_release_of<I: Into<Input>>(&mut self, input: I) {
        let index = Self::get_state_index(input.into());
        if self.states[index] {
            self.states[index] = false;
            self.releaste_timestamps[index] = Some(Instant::now());
        }
    }

    fn get_state_index(input: Input) -> usize {
        match input {
            Input::Keyboard(key) => key as usize,
            Input::MouseButton(button) => (button as usize) + 1 + MAX_KEY,
        }
    }

    fn check_within_duration(&self, input: Input, duration: Duration, is_pressed: bool) -> bool {
        let index = Self::get_state_index(input);

        let timestamps = match is_pressed {
            true => &self.press_timestamps,
            false => &self.releaste_timestamps,
        };

        // Check if the input timestamp is within 'duration' from now.
        if let Some(timestamp) = timestamps[index] {
            let input_duration = Instant::now() - timestamp;
            input_duration <= duration
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_check_fresh() {
        let input_state = InputState::new();
        assert!(!input_state.check_pressed(Key::A));
        assert!(input_state.check_released(Key::A));
    }

    #[test]
    fn test_check_when_pressed() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(Key::A);
        assert!(input_state.check_pressed(Key::A));
        assert!(!input_state.check_released(Key::A));
    }

    #[test]
    fn test_check_when_released() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(Key::A);
        input_state.signal_release_of(Key::A);
        assert!(!input_state.check_pressed(Key::A));
        assert!(input_state.check_released(Key::A));
    }

    #[test]
    fn test_check_when_pressed_within() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(MouseButton::Left);
        sleep(Duration::from_millis(50));
        assert!(input_state.check_pressed_within(MouseButton::Left, Duration::from_millis(75)));
        sleep(Duration::from_millis(50));
        assert!(!input_state.check_pressed_within(MouseButton::Left, Duration::from_millis(75)));
    }

    #[test]
    fn test_check_when_released_within() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(MouseButton::Left);
        input_state.signal_release_of(MouseButton::Left);
        sleep(Duration::from_millis(50));
        assert!(input_state.check_released_within(MouseButton::Left, Duration::from_millis(75)));
        sleep(Duration::from_millis(50));
        assert!(!input_state.check_released_within(MouseButton::Left, Duration::from_millis(75)));
    }
}
