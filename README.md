# Keybee

Keybee is a semantic input binding library. **It's currently an early proof of
concept that has been extracted from a larger game project, modified, and not
well tested.**

Keybee is built around straightforward core primitives:
- `Session` holds all state for an application
- `Action` defines anything a player can do, like jump or move
- `ActionSet` groups together related actions
- `Bindings` assigns inputs to actions

### Features
- `winit`: Enable support for winit events
- `gilrs`: Enable support for gil-rs events

### Getting Started
```rust
use keybee::{Session, ButtonAction, Clamped, Axis2dAction};

let session = keybee::Session::new();

let gameplay = session.create_action_set("gameplay");
let jump = gameplay.create_action::<ButtonAction>("jump");
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
    if translate != (0.0, 0.0) {
        println!("Player movement vector: {:?}", translate);
    }

    // At the end of every game tick, run `end_update` to reset relative axis
    // movements and button presses.
    session.end_update();
}
```

### Future Improvements
- Support for multiple players
- Other backends: SDL, others

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
