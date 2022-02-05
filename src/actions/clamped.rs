use crate::{bindings::Binding, state::InputState};

use super::ActionKind;

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
