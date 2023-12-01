use anyhow::bail;
use winit::event::{DeviceEvent, ElementState, Event as WinitEvent, MouseScrollDelta, WindowEvent};
use winit::keyboard::PhysicalKey;

use crate::{Button, KeyboardKey, MouseButton};

use super::Event;

impl<T> TryFrom<&WinitEvent<T>> for Event {
    type Error = anyhow::Error;

    fn try_from(event: &WinitEvent<T>) -> Result<Self, Self::Error> {
        match event {
            WinitEvent::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => match event.physical_key {
                PhysicalKey::Code(keycode) => {
                    let key = KeyboardKey::try_from(keycode)?;

                    match event.state {
                        ElementState::Pressed => Ok(Event::ButtonPressed(Button::Keyboard(key))),
                        ElementState::Released => Ok(Event::ButtonReleased(Button::Keyboard(key))),
                    }
                }
                _ => bail!("cannot convert event"),
            },

            WinitEvent::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                let button = MouseButton::try_from(*button)?;

                match state {
                    ElementState::Pressed => Ok(Event::ButtonPressed(Button::Mouse(button))),
                    ElementState::Released => Ok(Event::ButtonReleased(Button::Mouse(button))),
                }
            }

            WinitEvent::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => Ok(Event::CursorMoved(position.x as f32, position.y as f32)),

            WinitEvent::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => Ok(Event::MouseMotion(delta.0 as f32, delta.1 as f32)),

            WinitEvent::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(x, y) => Ok(Event::MouseWheel(x * 16.0, y * 16.0)),
                MouseScrollDelta::PixelDelta(offset) => {
                    Ok(Event::MouseWheel(offset.x as f32, offset.y as f32))
                }
            },

            _ => bail!("cannot convert event"),
        }
    }
}

impl<T> TryFrom<WinitEvent<T>> for Event {
    type Error = anyhow::Error;

    fn try_from(event: WinitEvent<T>) -> Result<Self, Self::Error> {
        <&WinitEvent<T>>::try_into(&event)
    }
}
