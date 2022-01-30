use std::collections::{HashMap, HashSet};

use crate::{
    buttons::{Axis1d, Axis2d, Button, MouseAxis1d},
    Event,
};

pub struct InputState {
    buttons: HashMap<Button, ButtonState>,

    mouse_motion: (f32, f32),
    cursor_position: (f32, f32),
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
    JustPressed,
    JustReleased,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),

            mouse_motion: (0.0, 0.0),
            cursor_position: (0.0, 0.0),
        }
    }

    pub fn button_state(&self, button: Button) -> ButtonState {
        self.buttons
            .get(&button)
            .copied()
            .unwrap_or(ButtonState::Released)
    }

    pub fn is_button_just_down(&self, button: Button) -> bool {
        matches!(self.buttons.get(&button), Some(ButtonState::JustPressed))
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        matches!(
            self.buttons.get(&button),
            Some(ButtonState::Pressed | ButtonState::JustPressed)
        )
    }

    pub fn get_axis1d(&self, axis: Axis1d) -> f32 {
        match axis {
            Axis1d::Mouse(mouse) => match mouse {
                MouseAxis1d::X => self.mouse_motion.0,
                MouseAxis1d::Y => self.mouse_motion.1,
            },
            Axis1d::Gamepad(_) => todo!(),
        }
    }

    pub fn get_axis2d(&self, axis: Axis2d) -> (f32, f32) {
        match axis {
            Axis2d::Mouse(_) => self.mouse_motion,
            Axis2d::Gamepad(_) => todo!(),
        }
    }

    pub fn cursor_position(&self) -> (f32, f32) {
        self.cursor_position
    }

    pub fn end_update(&mut self) {
        self.mouse_motion = (0.0, 0.0);

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

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::ButtonPressed(button) => {
                self.buttons.insert(button, ButtonState::JustPressed);
            }
            Event::ButtonReleased(button) => {
                self.buttons.insert(button, ButtonState::JustReleased);
            }
            Event::CursorMoved(x, y) => {
                self.cursor_position = (x, y);
            }
            Event::MouseMotion(x, y) => {
                self.mouse_motion = (self.mouse_motion.0 + x, self.mouse_motion.1 + y);
            }
        }
    }
}
