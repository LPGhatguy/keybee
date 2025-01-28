use sdl2::event::Event as SdlEvent;

use crate::{Button, KeyboardKey, MouseButton};

use super::Event;

impl Event {
    pub fn from_sdl2(event: &SdlEvent) -> Vec<Self> {
        match event {
            SdlEvent::MouseButtonDown { mouse_btn, .. } => {
                let Some(button) = MouseButton::from_sdl2(*mouse_btn) else {
                    return Vec::new();
                };

                vec![Event::ButtonPressed(Button::Mouse(button))]
            }

            SdlEvent::MouseButtonUp { mouse_btn, .. } => {
                let Some(button) = MouseButton::from_sdl2(*mouse_btn) else {
                    return Vec::new();
                };

                vec![Event::ButtonReleased(Button::Mouse(button))]
            }

            SdlEvent::MouseMotion {
                x, y, xrel, yrel, ..
            } => vec![
                Event::MouseMotion(*xrel as f32, *yrel as f32),
                Event::CursorMoved(*x as f32, *y as f32),
            ],

            SdlEvent::MouseWheel {
                precise_x,
                precise_y,
                ..
            } => vec![Event::MouseWheel(*precise_x * 16.0, *precise_y * 16.0)],

            SdlEvent::KeyDown { scancode, .. } => {
                let Some(key) = scancode.and_then(KeyboardKey::from_sdl2) else {
                    return Vec::new();
                };

                vec![Event::ButtonPressed(Button::Keyboard(key))]
            }

            SdlEvent::KeyUp { scancode, .. } => {
                let Some(key) = scancode.and_then(KeyboardKey::from_sdl2) else {
                    return Vec::new();
                };

                vec![Event::ButtonReleased(Button::Keyboard(key))]
            }

            _ => Vec::new(),
        }
    }
}
