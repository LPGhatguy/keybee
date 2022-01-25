use crate::{bindings::Binding, state::InputState};

use super::ActionKind;

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

impl Clamp for (f32, f32) {
    fn clamp(self, max: f32) -> Self {
        let (x, y) = self;
        let len = (x.powi(2) + y.powi(2)).sqrt();

        if len > max {
            (max * x / len, max * y / len)
        } else {
            self
        }
    }
}

impl Clamp for (f32, f32, f32) {
    fn clamp(self, max: f32) -> Self {
        let (x, y, z) = self;
        let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

        if len > max {
            (max * x / len, max * y / len, max * z / len)
        } else {
            self
        }
    }
}
