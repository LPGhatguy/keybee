#![doc = include_str!("../README.md")]

mod actions;
mod bindings;
mod buttons;
mod session;
mod state;

pub use crate::actions::{
    ActionKind, Axis1dAction, Axis2dAction, Axis3dAction, ButtonAction, ButtonHeldAction, Clamped,
};
pub use crate::buttons::*;
pub use crate::session::{Action, ActionSet, Session};
