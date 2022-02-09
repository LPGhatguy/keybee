use crate::bindings::Binding;
use crate::state::InputState;

use super::ActionKind;

/// Clamps an axis action, clamping the length of its values to a maximum of
/// 1.0.
#[derive(Debug)]
pub struct Clamped<T>(T);

impl<T> ActionKind for Clamped<T>
where
    T: ActionKind,
    T::Output: Clamp,
{
    type Output = T::Output;

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output> {
        T::get(state, binding).map(|v| v.clamp(1.0))
    }

    fn reduce(inputs: &[Self::Output]) -> Self::Output {
        T::reduce(inputs).clamp(1.0)
    }
}

pub trait Clamp {
    fn clamp(self, max: f32) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, max: f32) -> Self {
        self.clamp(-max, max)
    }
}

impl<const N: usize> Clamp for [f32; N] {
    fn clamp(self, max: f32) -> Self {
        let len = self.map(|v| v.powi(2)).into_iter().sum::<f32>().sqrt();

        if len > max {
            self.map(|v| v * max / len)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Axis1d, Axis1dAction, Axis1dBinding, Event, GamepadAxis1d};

    use super::*;

    // This will be used for 2D axis tests!
    #[allow(unused)]
    fn len2(a: f32, b: f32) -> f32 {
        (a.powi(2) + b.powi(2)).sqrt()
    }

    #[test]
    #[ignore = "axis events are currentl unimplemented"]
    fn clamp_f32() {
        let mut state = InputState::new();
        let axis = Axis1d::Gamepad(GamepadAxis1d::LeftStickX);
        let binding = Binding::Axis1d(Axis1dBinding::Axis {
            axis,
            sensitivity: 1.0,
        });
        type Action = Clamped<Axis1dAction>;

        assert_eq!(Action::get(&state, &binding), Some(0.0));

        state.handle_event(Event::Axis1dChanged(axis, 1.0));
        assert_eq!(Action::get(&state, &binding), Some(1.0));

        state.handle_event(Event::Axis1dChanged(axis, -1.0));
        assert_eq!(Action::get(&state, &binding), Some(-1.0));

        state.handle_event(Event::Axis1dChanged(axis, 0.5));
        assert_eq!(Action::get(&state, &binding), Some(0.5));

        state.handle_event(Event::Axis1dChanged(axis, 2.0));
        assert_eq!(Action::get(&state, &binding), Some(1.0));

        state.handle_event(Event::Axis1dChanged(axis, -3.5));
        assert_eq!(Action::get(&state, &binding), Some(-1.0));
    }
}
