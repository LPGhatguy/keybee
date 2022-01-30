use crate::Button;

#[cfg(feature = "gilrs")]
mod gilrs;

#[cfg(feature = "winit")]
mod winit;

/// Represents an input event that can be processed by the library.
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    ButtonPressed(Button),
    ButtonReleased(Button),
    CursorMoved(f32, f32),
    MouseMotion(f32, f32),
}
