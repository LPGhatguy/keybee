/*!
Keybee is a semantic input binding library. **It's currently an early proof of
concept that has been extracted from a larger game project, modified, and not
well tested.**

Keybee is built around straightforward core primitives:
- `Session` holds all state for an application
- `Action` defines anything a player can do, like jump or move
- `ActionSet` groups together related actions
- `Bindings` assigns inputs to actions

## Features
- `winit`: Enable support for winit events
- `gilrs`: Enable support for gil-rs events

## Getting Started
```rust,no_run
use keybee::{Session, EventAction, Clamped, Axis2dAction};

let session = keybee::Session::new();

let gameplay = session.create_action_set("gameplay");
let jump = gameplay.create_action::<EventAction>("jump");
let movement = gameplay.create_action::<Clamped<Axis2dAction>>("movement");

// Keybee will have support for deserializing bindings from files, but for now,
// this isn't quite done.
session.use_bindings(todo!("load bindings from somewhere"));

loop {
    // With the `winit` feature enabled:
    // session.handle_winit_event::<()>(todo!("pass winit events"));

    // With the `gilrs` feature enabled:
    // session.handle_gilrs_event(todo!("pass gil-rs events"));

    if jump.get() {
        println!("Player jumped!");
    }

    let translate = movement.get();
    if translate != [0.0, 0.0] {
        println!("Player movement vector: {:?}", translate);
    }

    // At the end of every game tick, run `end_update` to reset relative axis
    // movements and button presses.
    session.end_update();
}
```

## Future Improvements
- Support for multiple players
- Other backends: SDL, others
*/

mod actions;
mod bindings;
mod buttons;
mod session;
mod state;

pub use crate::actions::{
    ActionKind, Axis1dAction, Axis2dAction, Axis3dAction, BoolAction, Clamped, EventAction,
};
pub use crate::buttons::*;
pub use crate::session::{Action, ActionSet, Session};
