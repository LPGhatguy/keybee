use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Axis1d, Axis2d, Button};

#[derive(Debug, Clone, Copy)]
pub enum Binding {
    Button(Button),
    Axis1d(Axis1dBinding),
    Axis2d(Axis2dBinding),
    Axis3d(Axis3dBinding),
}

impl Serialize for Binding {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Binding {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Axis1dBinding {
    Buttons {
        neg: Button,
        pos: Button,
        sensitivity: f32,
    },
    Axis {
        axis: Axis1d,
        sensitivity: f32,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Axis2dBinding {
    Individual { x: Axis1dBinding, y: Axis1dBinding },
    Axis { axis: Axis2d, sensitivity: f32 },
}

#[derive(Debug, Clone, Copy)]
pub enum Axis3dBinding {
    Individual {
        x: Axis1dBinding,
        y: Axis1dBinding,
        z: Axis1dBinding,
    },
}
