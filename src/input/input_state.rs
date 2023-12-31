use std::time::{ Instant, Duration };
use super::{ inputs::{ INPUTS, MAX_KEY }, Keyboard, Input, Mouse };

/// Manages storing the current state of all the applications possible inputs.
pub struct InputState {
    // important to note that a state is only valid if it has a
    // non-None timestamp.
    states: [bool; INPUTS],
    press_timestamps: [Option<Instant>; INPUTS],
    releaste_timestamps: [Option<Instant>; INPUTS],
}

impl From<Keyboard> for Input {
    fn from(val: Keyboard) -> Self {
        Input::Keyboard(val)
    }
}

impl From<Mouse> for Input {
    fn from(val: Mouse) -> Self {
        Input::Mouse(val)
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

    /// Signals to the [InputState] that a specific [input] was pressed.
    ///
    /// Will ignore if the [input] is already pressed.
    pub fn signal_press_of<I: Into<Input>>(&mut self, input: I) {
        let index = Self::get_state_index(input.into());
        if !self.states[index] {
            self.states[index] = true;
            self.press_timestamps[index] = Some(Instant::now());
        }
    }

    /// Signals to the [InputState] that a specific [input] was released.
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
            Input::Mouse(input) => (input as usize) + 1 + MAX_KEY,
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
        assert!(!input_state.check_pressed(Keyboard::A));
        assert!(input_state.check_released(Keyboard::A));
    }

    #[test]
    fn test_check_when_pressed() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(Keyboard::A);
        assert!(input_state.check_pressed(Keyboard::A));
        assert!(!input_state.check_released(Keyboard::A));
    }

    #[test]
    fn test_check_when_released() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(Keyboard::A);
        input_state.signal_release_of(Keyboard::A);
        assert!(!input_state.check_pressed(Keyboard::A));
        assert!(input_state.check_released(Keyboard::A));
    }

    #[test]
    fn test_check_when_pressed_within() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(Mouse::Left);
        sleep(Duration::from_millis(50));
        assert!(input_state.check_pressed_within(Mouse::Left, Duration::from_millis(75)));
        sleep(Duration::from_millis(50));
        assert!(!input_state.check_pressed_within(Mouse::Left, Duration::from_millis(75)));
    }

    #[test]
    fn test_check_when_released_within() {
        let mut input_state = InputState::new();
        input_state.signal_press_of(Mouse::Left);
        input_state.signal_release_of(Mouse::Left);
        sleep(Duration::from_millis(50));
        assert!(input_state.check_released_within(Mouse::Left, Duration::from_millis(75)));
        sleep(Duration::from_millis(50));
        assert!(!input_state.check_released_within(Mouse::Left, Duration::from_millis(75)));
    }
}
