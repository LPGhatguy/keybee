#![allow(missing_docs)]

#[macro_use]
mod macros;

#[cfg(feature = "winit")]
use winit::event::MouseButton as WinitMouseButton;

wrapper_enum! {
    Button {
        Keyboard(KeyboardKey, keyboard),
        Mouse(MouseButton, mouse),
        Gamepad(GamepadButton, gamepad),
    }
}

wrapper_enum! {
    Axis1d {
        Mouse(MouseAxis1d, mouse),
        Gamepad(GamepadAxis1d, gamepad),
    }
}

wrapper_enum! {
    Axis2d {
        Mouse(MouseAxis2d, mouse),
        Gamepad(GamepadAxis2d, gamepad),
    }
}

wrapper_enum! {
    Input {
        Keyboard(KeyboardKey, keyboard),
        Mouse(Mouse, mouse),
        Gamepad(Gamepad, gamepad),
    }
}

keyboard! {
    A(a),
    B(b),
    C(c),
    D(d),
    E(e),
    F(f),
    G(g),
    H(h),
    I(i),
    J(j),
    K(k),
    L(l),
    M(m),
    N(n),
    O(o),
    P(p),
    Q(q),
    R(r),
    S(s),
    T(t),
    U(u),
    V(v),
    W(w),
    X(x),
    Y(y),
    Z(z),

    Zero(zero),
    One(one),
    Two(two),
    Three(three),
    Four(four),
    Five(five),
    Six(six),
    Seven(seven),
    Eight(eight),
    Nine(nine),

    F1(f1),
    F2(f2),
    F3(f3),
    F4(f4),
    F5(f5),
    F6(f6),
    F7(f7),
    F8(f8),
    F9(f9),
    F10(f10),
    F11(f11),
    F12(f12),
    F13(f13),
    F14(f14),
    F15(f15),
    F16(f16),
    F17(f17),
    F18(f18),
    F19(f19),
    F20(f20),
    F21(f21),
    F22(f22),
    F23(f23),
    F24(f24),

    Up(up),
    Down(down),
    Left(left),
    Right(right),

    Space(space),
    LShift(leftshift),
    RShift(rightshift),
    LControl(leftctrl),
    RControl(rightctrl),
    LAlt(leftalt),
    RAlt(rightalt),
    Escape(escape),
}

keyboard_winit! {
    A => A,
    B => B,
    C => C,
    D => D,
    E => E,
    F => F,
    G => G,
    H => H,
    I => I,
    J => J,
    K => K,
    L => L,
    M => M,
    N => N,
    O => O,
    P => P,
    Q => Q,
    R => R,
    S => S,
    T => T,
    U => U,
    V => V,
    W => W,
    X => X,
    Y => Y,
    Z => Z,

    Zero => Key0,
    One => Key1,
    Two => Key2,
    Three => Key3,
    Four => Key4,
    Five => Key5,
    Six => Key6,
    Seven => Key7,
    Eight => Key8,
    Nine => Key9,

    F1 => F1,
    F2 => F2,
    F3 => F3,
    F4 => F4,
    F5 => F5,
    F6 => F6,
    F7 => F7,
    F8 => F8,
    F9 => F9,
    F10 => F10,
    F11 => F11,
    F12 => F12,
    F13 => F13,
    F14 => F14,
    F15 => F15,
    F16 => F16,
    F17 => F17,
    F18 => F18,
    F19 => F19,
    F20 => F20,
    F21 => F21,
    F22 => F22,
    F23 => F23,
    F24 => F24,

    Up => Up,
    Down => Down,
    Left => Left,
    Right => Right,

    Space => Space,
    LShift => LShift,
    RShift => RShift,
    LControl => LControl,
    RControl => RControl,
    LAlt => LAlt,
    RAlt => RAlt,
    Escape => Escape,
}

define_device! {
    Gamepad(gamepad)

    Button(GamepadButton {
        A(a),
        B(b),
        X(x),
        Y(y),
        DpadUp(dpadup),
        DpadDown(dpaddown),
        DpadLeft(dpadleft),
        DpadRight(dpadright),

        LeftStickLeft(leftstickleft),
        LeftStickRight(leftstickright),
        LeftStickUp(leftstickup),
        LeftStickDown(leftstickdown),

        RightStickLeft(rightstickleft),
        RightStickRight(rightstickright),
        RightStickUp(rightstickup),
        RightStickDown(rightstickdown),

        LeftShoulder(leftshoulder),
        RightShoulder(rightshoulder),
        LeftTrigger(lefttrigger),
        RightTrigger(righttrigger),
        LeftThumb(leftthumb),
        RightThumb(rightthumb),
        Select(select),
        Start(start),
    })

    Axis1d(GamepadAxis1d {
        LeftStickX(leftstickx),
        LeftStickY(leftsticky),
        RightStickX(rightstickx),
        RightStickY(rightsticky),
        LeftTrigger(lefttrigger),
        RightTrigger(righttrigger),
        DpadX(dpadx),
        DpadY(dpady),
    })

    Axis2d(GamepadAxis2d {
        LeftStick(leftstick),
        RightStick(rightstick),
    })
}

#[cfg(feature = "gilrs")]
impl TryFrom<gilrs::Button> for GamepadButton {
    type Error = anyhow::Error;

    fn try_from(value: gilrs::Button) -> Result<Self, Self::Error> {
        use gilrs::Button::*;

        match value {
            South => Ok(Self::A),
            East => Ok(Self::B),
            North => Ok(Self::Y),
            West => Ok(Self::X),
            LeftTrigger => Ok(Self::LeftShoulder),
            LeftTrigger2 => Ok(Self::LeftTrigger),
            RightTrigger => Ok(Self::RightShoulder),
            RightTrigger2 => Ok(Self::RightTrigger),
            Select => Ok(Self::Select),
            Start => Ok(Self::Start),
            LeftThumb => Ok(Self::LeftThumb),
            RightThumb => Ok(Self::RightThumb),
            DPadUp => Ok(Self::DpadUp),
            DPadDown => Ok(Self::DpadDown),
            DPadLeft => Ok(Self::DpadLeft),
            DPadRight => Ok(Self::DpadRight),
            _ => anyhow::bail!("gilrs button {:?} is not supported", value),
        }
    }
}

#[cfg(feature = "gilrs")]
impl TryFrom<gilrs::Axis> for GamepadAxis1d {
    type Error = anyhow::Error;

    fn try_from(value: gilrs::Axis) -> Result<Self, Self::Error> {
        use gilrs::Axis::*;

        match value {
            LeftStickX => Ok(Self::LeftStickX),
            LeftStickY => Ok(Self::LeftStickY),
            LeftZ => Ok(Self::LeftTrigger),
            RightStickX => Ok(Self::RightStickX),
            RightStickY => Ok(Self::RightStickY),
            RightZ => Ok(Self::RightTrigger),
            DPadX => Ok(Self::DpadX),
            DPadY => Ok(Self::DpadY),
            _ => anyhow::bail!("gilrs axis {:?} is not supported", value),
        }
    }
}

define_device! {
    Mouse(mouse)

    Button(MouseButton {
        Button1(button1),
        Button2(button2),
        Button3(button3),
        Button4(button4),
        Button5(button5),
    })

    Axis1d(MouseAxis1d {
        X(x),
        Y(y),
    })

    Axis2d(MouseAxis2d {
        XY(xy),
    })
}

#[cfg(feature = "winit")]
impl TryFrom<WinitMouseButton> for MouseButton {
    type Error = anyhow::Error;

    fn try_from(value: WinitMouseButton) -> Result<Self, Self::Error> {
        match value {
            WinitMouseButton::Left => Ok(Self::Button1),
            WinitMouseButton::Right => Ok(Self::Button2),
            WinitMouseButton::Middle => Ok(Self::Button3),
            WinitMouseButton::Other(4) => Ok(Self::Button4),
            WinitMouseButton::Other(5) => Ok(Self::Button5),
            _ => anyhow::bail!("mouse button {:?} is not supported by keybee", value),
        }
    }
}
