use std::collections::{HashMap, HashSet};

use crate::buttons::{Axis1d, Axis2d, Button, MouseAxis1d};
use crate::event::Event;

/// Tracks all of the state for a Keybee session, like what buttons are down or
/// where the cursor is.
#[derive(Debug)]
pub struct InputState {
    buttons: HashMap<Button, ButtonState>,
    mouse_motion: [f32; 2],
    cursor_position: [f32; 2],
    viewport_position: [f32; 2],
}

/// The current state of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// The button was pressed this update.
    JustPressed,

    /// The button was released this update.
    JustReleased,

    /// The button has been pressed since before this update.
    Pressed,

    /// The button has been released since before this update.
    Released,
}

impl InputState {
    /// Create a new `InputState`.
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),

            mouse_motion: [0.0, 0.0],
            cursor_position: [0.0, 0.0],
            viewport_position: [0.0, 0.0],
        }
    }

    /// Sets the offset of the game viewport, used for reporting the cursor's
    /// position.
    pub fn set_viewport_position<P: Into<[f32; 2]>>(&mut self, pos: P) {
        self.viewport_position = pos.into();
    }

    /// Returns the current state for the given button.
    pub fn button_state<B: Into<Button>>(&self, button: B) -> ButtonState {
        self.buttons
            .get(&button.into())
            .copied()
            .unwrap_or(ButtonState::Released)
    }

    /// Tells whether the given button was pressed this update.
    pub fn is_button_just_down<B: Into<Button>>(&self, button: B) -> bool {
        let button = button.into();

        matches!(self.buttons.get(&button), Some(ButtonState::JustPressed))
    }

    /// Tells whether the given button was released this update.
    pub fn is_button_just_up<B: Into<Button>>(&self, button: B) -> bool {
        let button = button.into();

        matches!(self.buttons.get(&button), Some(ButtonState::JustReleased))
    }

    /// Tells whether the given button is currently pressed.
    pub fn is_button_down<B: Into<Button>>(&self, button: B) -> bool {
        let button = button.into();

        matches!(
            self.buttons.get(&button),
            Some(ButtonState::Pressed | ButtonState::JustPressed)
        )
    }

    /// Tells whether the given button is currently released.
    pub fn is_button_up<B: Into<Button>>(&self, button: B) -> bool {
        let button = button.into();

        matches!(
            self.buttons.get(&button),
            None | Some(ButtonState::Released | ButtonState::JustReleased)
        )
    }

    /// Tells the state of the given axis.
    pub fn get_axis1d(&self, axis: Axis1d) -> f32 {
        match axis {
            Axis1d::Mouse(mouse) => match mouse {
                MouseAxis1d::X => self.mouse_motion[0],
                MouseAxis1d::Y => self.mouse_motion[1],
            },
            Axis1d::Gamepad(_) => todo!(),
        }
    }

    /// Tells the state of the given axis.
    pub fn get_axis2d(&self, axis: Axis2d) -> [f32; 2] {
        match axis {
            Axis2d::Mouse(_) => self.mouse_motion,
            Axis2d::Gamepad(_) => todo!(),
        }
    }

    pub fn mouse_motion(&self) -> [f32; 2] {
        self.mouse_motion
    }

    /// Tells the current position of the cursor.
    pub fn cursor_position(&self) -> [f32; 2] {
        let [x, y] = self.cursor_position;
        let [vx, vy] = self.viewport_position;
        [x - vx, y - vy]
    }

    /// Marks the end of an update, resetting accumulated mouse motion and
    /// processing buttons being pressed or released.
    pub fn end_update(&mut self) {
        self.mouse_motion = [0.0, 0.0];

        let mut to_remove = HashSet::new();

        for (button, state) in self.buttons.iter_mut() {
            match state {
                ButtonState::JustPressed => *state = ButtonState::Pressed,
                ButtonState::JustReleased => {
                    to_remove.insert(*button);
                }
                _ => {}
            }
        }

        for button in to_remove {
            self.buttons.remove(&button);
        }
    }

    /// Handle the given event and update input state accordingly.
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::ButtonPressed(button) => {
                self.buttons.insert(button, ButtonState::JustPressed);
            }
            Event::ButtonReleased(button) => {
                self.buttons.insert(button, ButtonState::JustReleased);
            }
            Event::Axis1dChanged(_axis, _value) => todo!(),
            Event::Axis2dChanged(_axis, _value) => todo!(),
            Event::CursorMoved(x, y) => {
                self.cursor_position = [x, y];
            }
            Event::MouseMotion(x, y) => {
                self.mouse_motion = [self.mouse_motion[0] + x, self.mouse_motion[1] + y];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::KeyboardKey;

    use super::*;

    #[test]
    fn button_state_transition() {
        let mut state = InputState::new();
        assert_eq!(state.button_state(KeyboardKey::W), ButtonState::Released);

        state.handle_event(Event::ButtonPressed(KeyboardKey::W.into()));
        assert_eq!(state.button_state(KeyboardKey::W), ButtonState::JustPressed);

        state.end_update();
        assert_eq!(state.button_state(KeyboardKey::W), ButtonState::Pressed);

        state.handle_event(Event::ButtonReleased(KeyboardKey::W.into()));
        assert_eq!(
            state.button_state(KeyboardKey::W),
            ButtonState::JustReleased
        );

        state.end_update();
        assert_eq!(state.button_state(KeyboardKey::W), ButtonState::Released);
    }

    #[test]
    fn mouse_motion_accumulation() {
        let mut state = InputState::new();
        assert_eq!(state.mouse_motion(), [0.0, 0.0]);

        state.handle_event(Event::MouseMotion(5.0, 5.0));
        state.handle_event(Event::MouseMotion(3.0, 2.0));
        assert_eq!(state.mouse_motion(), [8.0, 7.0]);

        state.end_update();
        assert_eq!(state.mouse_motion(), [0.0, 0.0]);
    }
}
