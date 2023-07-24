use std::time::{ Instant, Duration };
use super::{ button::{ BUTTONS, MAX_KEY }, Keyboard, Button, Mouse };

/// Manages storring the current state of an application user's inputs.
pub struct InputState {
    // important to note that a state is only valid if it has a
    // non-None timestamp.
    states: [bool; BUTTONS],
    press_timestamps: [Option<Instant>; BUTTONS],
    releaste_timestamps: [Option<Instant>; BUTTONS],
}

impl From<Keyboard> for Button {
    fn from(val: Keyboard) -> Self {
        Button::Keyboard(val)
    }
}

impl From<Mouse> for Button {
    fn from(val: Mouse) -> Self {
        Button::Mouse(val)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            states: [false; BUTTONS],
            press_timestamps: [None; BUTTONS],
            releaste_timestamps: [None; BUTTONS],
        }
    }
}

impl InputState {
    /// Creates a fresh [InputState].
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if an [Button] is currently pressed.
    pub fn check_pressed<I: Into<Button>>(&self, button: I) -> bool {
        self.states[Self::get_state_index(button.into())]
    }

    /// Checks if an [Button] is currently released.
    pub fn check_released<I: Into<Button>>(&self, button: I) -> bool {
        !self.states[Self::get_state_index(button.into())]
    }

    /// Checks if an [Button] was pressed within a [Duration].
    ///
    /// This is useful for checking if an [Button] was just pressed rather than if it is held
    /// down.
    pub fn check_pressed_within<I: Into<Button>>(&self, button: I, duration: Duration) -> bool {
        self.check_within_duration(button.into(), duration, true)
    }

    /// Checks if an [Button] was released within a [Duration].
    ///
    /// This is useful for checking if an [Button] was just released rather than if it isn't
    /// held down.
    pub fn check_released_within<I: Into<Button>>(&self, button: I, duration: Duration) -> bool {
        self.check_within_duration(button.into(), duration, false)
    }

    /// Signals to the [InputState] that a specific [Button] was pressed.
    ///
    /// Will ignore if the [Button] is already pressed.
    pub fn signal_press_of<I: Into<Button>>(&mut self, button: I) {
        let index = Self::get_state_index(button.into());
        if !self.states[index] {
            self.states[index] = true;
            self.press_timestamps[index] = Some(Instant::now());
        }
    }

    /// Signals to the [InputState] that a specific [Button] was released.
    pub fn signal_release_of<I: Into<Button>>(&mut self, button: I) {
        let index = Self::get_state_index(button.into());
        if self.states[index] {
            self.states[index] = false;
            self.releaste_timestamps[index] = Some(Instant::now());
        }
    }

    fn get_state_index(button: Button) -> usize {
        match button {
            Button::Keyboard(key) => key as usize,
            Button::Mouse(button) => (button as usize) + 1 + MAX_KEY,
        }
    }

    fn check_within_duration(&self, button: Button, duration: Duration, is_pressed: bool) -> bool {
        let index = Self::get_state_index(button);

        let timestamps = match is_pressed {
            true => &self.press_timestamps,
            false => &self.releaste_timestamps,
        };

        // Check if the Button timestamp is within 'duration' from now.
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
