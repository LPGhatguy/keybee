use anyhow::bail;
use gilrs::EventType;

use crate::{Button, Event, GamepadButton};

impl TryFrom<&gilrs::Event> for Event {
    type Error = anyhow::Error;

    fn try_from(event: &gilrs::Event) -> Result<Event, Self::Error> {
        match &event.event {
            EventType::ButtonPressed(button, _) => {
                let button = GamepadButton::try_from(*button)?;
                Ok(Event::ButtonPressed(Button::Gamepad(button)))
            }

            EventType::ButtonReleased(button, _) => {
                let button = GamepadButton::try_from(*button)?;
                Ok(Event::ButtonReleased(Button::Gamepad(button)))
            }

            _ => bail!("cannot convert event"),
        }
    }
}

impl TryFrom<gilrs::Event> for Event {
    type Error = anyhow::Error;

    fn try_from(event: gilrs::Event) -> Result<Event, Self::Error> {
        <&gilrs::Event>::try_into(&event)
    }
}
