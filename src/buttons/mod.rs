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
    Binding {
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
    Space(space),
    LShift(leftshift),
    LControl(leftctrl),
    LAlt(leftalt),
    Escape(escape),
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
