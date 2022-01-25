mod clamped;

use crate::bindings::{Axis1dBinding, Axis2dBinding, Axis3dBinding, Binding};
use crate::state::InputState;

pub use clamped::*;

pub trait ActionKind {
    type Output;

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output>;
    fn reduce(inputs: &[Self::Output]) -> Self::Output;
}

pub struct ButtonAction;

impl ActionKind for ButtonAction {
    type Output = bool;

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output> {
        let binding = match binding {
            Binding::Button(inner) => inner,
            _ => return None,
        };

        Some(state.is_button_just_down(*binding))
    }

    fn reduce(inputs: &[Self::Output]) -> Self::Output {
        inputs.iter().any(|x| *x)
    }
}

pub struct ButtonHeldAction;

impl ActionKind for ButtonHeldAction {
    type Output = bool;

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output> {
        let binding = match binding {
            Binding::Button(inner) => inner,
            _ => return None,
        };

        Some(state.is_button_down(*binding))
    }

    fn reduce(inputs: &[Self::Output]) -> Self::Output {
        inputs.iter().any(|x| *x)
    }
}

pub struct Axis1dAction;

impl ActionKind for Axis1dAction {
    type Output = f32;

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output> {
        let binding = match binding {
            Binding::Axis1d(inner) => inner,
            _ => return None,
        };

        match binding {
            Axis1dBinding::Buttons {
                neg,
                pos,
                sensitivity,
            } => {
                let is_neg = -(state.is_button_down(*neg) as u8 as f32);
                let is_pos = state.is_button_down(*pos) as u8 as f32;

                Some((is_neg + is_pos) * sensitivity)
            }
            Axis1dBinding::Axis { axis, sensitivity } => {
                Some(state.get_axis1d(*axis) * sensitivity)
            }
        }
    }

    fn reduce(inputs: &[Self::Output]) -> Self::Output {
        inputs.iter().sum::<f32>()
    }
}

pub struct Axis2dAction;

impl ActionKind for Axis2dAction {
    type Output = (f32, f32);

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output> {
        let binding = match binding {
            Binding::Axis2d(inner) => inner,
            _ => return None,
        };

        match binding {
            Axis2dBinding::Individual { x, y } => {
                let x = Axis1dAction::get(state, &Binding::Axis1d(*x))?;
                let y = Axis1dAction::get(state, &Binding::Axis1d(*y))?;

                Some((x, y))
            }
            Axis2dBinding::Axis { axis, sensitivity } => {
                let (x, y) = state.get_axis2d(*axis);

                Some((x * sensitivity, y * sensitivity))
            }
        }
    }

    fn reduce(inputs: &[Self::Output]) -> Self::Output {
        inputs
            .iter()
            .fold((0.0, 0.0), |(ax, ay), (bx, by)| (ax + bx, ay + by))
    }
}

pub struct Axis3dAction;

impl ActionKind for Axis3dAction {
    type Output = (f32, f32, f32);

    fn get(state: &InputState, binding: &Binding) -> Option<Self::Output> {
        let binding = match binding {
            Binding::Axis3d(inner) => inner,
            _ => return None,
        };

        match binding {
            Axis3dBinding::Individual { x, y, z } => {
                let x = Axis1dAction::get(state, &Binding::Axis1d(*x))?;
                let y = Axis1dAction::get(state, &Binding::Axis1d(*y))?;
                let z = Axis1dAction::get(state, &Binding::Axis1d(*z))?;

                Some((x, y, z))
            }
        }
    }

    fn reduce(inputs: &[Self::Output]) -> Self::Output {
        inputs
            .iter()
            .fold((0.0, 0.0, 0.0), |(ax, ay, az), (bx, by, bz)| {
                (ax + bx, ay + by, az + bz)
            })
    }
}
