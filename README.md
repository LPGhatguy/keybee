# keybee
keybee is a semantic input binding library. It's modeled roughly after OpenXR input.

## Features
- `winit`: Enable support for winit events
- `gilrs`: Enable support for gil-rs events

## Examples

Getting started:
```rust,no_run
use keybee::ButtonAction;

let session = keybee::Session::new();

let gameplay = session.create_action_set("gameplay");
let jump = gameplay.create_action::<ButtonAction>("jump");

session.use_bindings(todo!("load bindings from somewhere"));

loop {
	session.handle_winit_event::<()>(todo!("pass winit events"));
	session.handle_gilrs_event(todo!("pass gil-rs events"));

	if jump.get() {
		println!("Player jumped!");
	}

	session.end_update();
}
```
