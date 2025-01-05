use crate::{Axis1d, Axis2d, Button};

#[cfg(feature = "gilrs")]
mod gilrs;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "sdl2")]
mod sdl2;

/// Represents an input event that can be processed by the library.
///
/// By enabling the `winit` or `gilrs` features, keybee supports converting
/// events from those libraries to this event type through the [`TryFrom`] and
/// [`TryInto`] traits.
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    ButtonPressed(Button),
    ButtonReleased(Button),
    Axis1dChanged(Axis1d, f32),
    Axis2dChanged(Axis2d, [f32; 2]),
    CursorMoved(f32, f32),
    MouseMotion(f32, f32),
    MouseWheel(f32, f32),
}
