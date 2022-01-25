use std::collections::{HashMap, HashSet};

use crate::buttons::{Axis1d, Axis2d, Button, MouseAxis1d};

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

    #[cfg(feature = "gilrs")]
    pub fn handle_gilrs_event(&mut self, event: &gilrs::Event) {
        use crate::GamepadButton;
        use gilrs::EventType;

        match &event.event {
            EventType::ButtonPressed(button, _) => {
                if let Ok(button) = GamepadButton::try_from(*button) {
                    self.buttons
                        .insert(Button::Gamepad(button), ButtonState::JustPressed);
                }
            }

            EventType::ButtonReleased(button, _) => {
                if let Ok(button) = GamepadButton::try_from(*button) {
                    self.buttons
                        .insert(Button::Gamepad(button), ButtonState::JustReleased);
                }
            }

            _ => {}
        }
    }

    #[cfg(feature = "winit")]
    pub fn handle_winit_event<T>(&mut self, event: &winit::event::Event<T>) {
        use crate::{KeyboardKey, MouseButton};
        use winit::event::{DeviceEvent, ElementState, Event, WindowEvent};

        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input.virtual_keycode {
                Some(keycode) => {
                    if let Ok(key) = KeyboardKey::try_from(keycode) {
                        match input.state {
                            ElementState::Pressed => {
                                self.buttons
                                    .insert(Button::Keyboard(key), ButtonState::JustPressed);
                            }
                            ElementState::Released => {
                                self.buttons
                                    .insert(Button::Keyboard(key), ButtonState::JustReleased);
                            }
                        }
                    }
                }
                None => {}
            },

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if let Ok(button) = MouseButton::try_from(*button) {
                    match state {
                        ElementState::Pressed => {
                            self.buttons
                                .insert(Button::Mouse(button), ButtonState::JustPressed);
                        }
                        ElementState::Released => {
                            self.buttons
                                .insert(Button::Mouse(button), ButtonState::JustReleased);
                        }
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                self.cursor_position = (position.x as f32, position.y as f32);
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                self.mouse_motion.0 += delta.0 as f32;
                self.mouse_motion.1 += delta.1 as f32;
            }

            _ => {}
        }
    }
}
