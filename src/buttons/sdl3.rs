use sdl3::keyboard::Scancode;
use sdl3::mouse::MouseButton as SdlMouseButton;

use super::{KeyboardKey, MouseButton};

impl MouseButton {
    pub fn from_sdl3(sdl: SdlMouseButton) -> Option<Self> {
        match sdl {
            SdlMouseButton::Left => Some(Self::Button1),
            SdlMouseButton::Right => Some(Self::Button2),
            SdlMouseButton::Middle => Some(Self::Button3),
            SdlMouseButton::X1 => Some(Self::Button4),
            SdlMouseButton::X2 => Some(Self::Button5),
            _ => None,
        }
    }
}

impl KeyboardKey {
    pub fn from_sdl3(sdl: Scancode) -> Option<Self> {
        match sdl {
            Scancode::A => Some(Self::A),
            Scancode::B => Some(Self::B),
            Scancode::C => Some(Self::C),
            Scancode::D => Some(Self::D),
            Scancode::E => Some(Self::E),
            Scancode::F => Some(Self::F),
            Scancode::G => Some(Self::G),
            Scancode::H => Some(Self::H),
            Scancode::I => Some(Self::I),
            Scancode::J => Some(Self::J),
            Scancode::K => Some(Self::K),
            Scancode::L => Some(Self::L),
            Scancode::M => Some(Self::M),
            Scancode::N => Some(Self::N),
            Scancode::O => Some(Self::O),
            Scancode::P => Some(Self::P),
            Scancode::Q => Some(Self::Q),
            Scancode::R => Some(Self::R),
            Scancode::S => Some(Self::S),
            Scancode::T => Some(Self::T),
            Scancode::U => Some(Self::U),
            Scancode::V => Some(Self::V),
            Scancode::W => Some(Self::W),
            Scancode::X => Some(Self::X),
            Scancode::Y => Some(Self::Y),
            Scancode::Z => Some(Self::Z),

            Scancode::_0 => Some(Self::Zero),
            Scancode::_1 => Some(Self::One),
            Scancode::_2 => Some(Self::Two),
            Scancode::_3 => Some(Self::Three),
            Scancode::_4 => Some(Self::Four),
            Scancode::_5 => Some(Self::Five),
            Scancode::_6 => Some(Self::Six),
            Scancode::_7 => Some(Self::Seven),
            Scancode::_8 => Some(Self::Eight),
            Scancode::_9 => Some(Self::Nine),

            Scancode::F1 => Some(Self::F1),
            Scancode::F2 => Some(Self::F2),
            Scancode::F3 => Some(Self::F3),
            Scancode::F4 => Some(Self::F4),
            Scancode::F5 => Some(Self::F5),
            Scancode::F6 => Some(Self::F6),
            Scancode::F7 => Some(Self::F7),
            Scancode::F8 => Some(Self::F8),
            Scancode::F9 => Some(Self::F9),
            Scancode::F10 => Some(Self::F10),
            Scancode::F11 => Some(Self::F11),
            Scancode::F12 => Some(Self::F12),
            Scancode::F13 => Some(Self::F13),
            Scancode::F14 => Some(Self::F14),
            Scancode::F15 => Some(Self::F15),
            Scancode::F16 => Some(Self::F16),
            Scancode::F17 => Some(Self::F17),
            Scancode::F18 => Some(Self::F18),
            Scancode::F19 => Some(Self::F19),
            Scancode::F20 => Some(Self::F20),
            Scancode::F21 => Some(Self::F21),
            Scancode::F22 => Some(Self::F22),
            Scancode::F23 => Some(Self::F23),
            Scancode::F24 => Some(Self::F24),

            Scancode::Tab => Some(Self::Tab),
            Scancode::Return => Some(Self::Return),

            Scancode::Up => Some(Self::Up),
            Scancode::Down => Some(Self::Down),
            Scancode::Left => Some(Self::Left),
            Scancode::Right => Some(Self::Right),

            Scancode::Space => Some(Self::Space),
            Scancode::LShift => Some(Self::LShift),
            Scancode::RShift => Some(Self::RShift),
            Scancode::LCtrl => Some(Self::LControl),
            Scancode::RCtrl => Some(Self::RControl),
            Scancode::LAlt => Some(Self::LAlt),
            Scancode::RAlt => Some(Self::RAlt),
            Scancode::Escape => Some(Self::Escape),
            Scancode::Delete => Some(Self::Delete),

            _ => None,
        }
    }
}
