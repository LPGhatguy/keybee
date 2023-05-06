mod bindings_cache;

use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::actions::ActionKind;
use crate::bindings::Bindings;
use crate::state::InputState;
use crate::Event;

use self::bindings_cache::BindingsCache;

/// The main entrypoint for using Keybee. [`ActionSet`]s are created from a
/// `Session`, which can create [`Action`]s.
pub struct Session {
    inner: Arc<SessionInner>,
}

struct SessionInner {
    input: RwLock<InputState>,
    bindings: RwLock<Bindings>,
    bindings_cache: RwLock<BindingsCache>,
}

impl Session {
    /// Create a new Keybee Session.
    pub fn new() -> Self {
        let inner = Arc::new(SessionInner {
            input: RwLock::new(InputState::new()),
            bindings: RwLock::new(Bindings::new()),
            bindings_cache: RwLock::new(BindingsCache::new()),
        });

        Self { inner }
    }

    /// Create a new action set with the given name.
    #[must_use]
    pub fn create_action_set(&self, name: &str) -> ActionSet {
        ActionSet {
            session: self.inner.clone(),
            enabled: Arc::new(AtomicBool::new(true)),
            name: name.to_owned(),
        }
    }

    /// Clear all bindings in the session.
    pub fn clear_bindings(&self) {
        let mut bindings = self.inner.bindings.write();
        let mut bindings_cache = self.inner.bindings_cache.write();

        bindings.clear();
        bindings_cache.clear();
    }

    /// Apply the given bindings to the session, merging them with the existing
    /// set of bindings.
    pub fn use_bindings(&self, new: Bindings) {
        let mut bindings = self.inner.bindings.write();
        let mut bindings_cache = self.inner.bindings_cache.write();

        bindings.merge(new);
        bindings_cache.clear();

        for (set_name, action_set) in &bindings.action_sets {
            for (action_name, action_bindings) in &action_set.actions {
                let full_name = format!("{}/{}", set_name, action_name);
                bindings_cache.insert(full_name, action_bindings.clone());
            }
        }
    }

    /// Sets the offset of the game viewport, used for reporting the cursor's
    /// position.
    pub fn set_viewport_position<P>(&mut self, position: P)
    where
        P: Into<[f32; 2]>,
    {
        let mut input = self.inner.input.write();
        input.set_viewport_position(position);
    }

    /// Process an event and update the session's state.
    ///
    /// Enable the `winit` or `gilrs` features to let this method handle events
    /// from those crates.
    pub fn handle_event<E>(&mut self, event: E)
    where
        E: TryInto<Event>,
    {
        if let Ok(event) = event.try_into() {
            let mut input = self.inner.input.write();
            input.handle_event(event);
        }
    }

    /// Indicate to Keybee that a game update has just run. This resets any
    /// edge-triggered inputs like buttons or mouse motion.
    pub fn end_update(&mut self) {
        let mut input = self.inner.input.write();
        input.end_update();
    }

    pub fn state(&mut self) -> impl Deref<Target = InputState> + '_ {
        self.inner.input.read()
    }
}

/// Defines a group of actions that a player can perform.
///
/// Created with [`Session::create_action_set`].
pub struct ActionSet {
    session: Arc<SessionInner>,
    enabled: Arc<AtomicBool>,
    name: String,
}

impl ActionSet {
    /// Create a new action that can be activated by the player.
    #[must_use]
    pub fn create_action<K: ActionKind>(&self, name: &str, action: K) -> Action<K> {
        let full_name = format!("{}/{}", self.name, name);

        Action {
            session: self.session.clone(),
            set_enabled: Arc::clone(&self.enabled),
            storage: RwLock::new(action),
            full_name,
            _phantom: PhantomData,
        }
    }

    /// Enable or disable all actions within this action set.
    pub fn set_enabled(&self, value: bool) {
        self.enabled.store(value, Ordering::SeqCst);
    }

    /// Tells whether the action set is currently enabled.
    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// Returns the name of the action set given when it was created.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Defines something that a player can do.
///
/// Created with [`ActionSet::create_action`].
pub struct Action<K> {
    session: Arc<SessionInner>,
    set_enabled: Arc<AtomicBool>,
    storage: RwLock<K>,
    full_name: String,
    _phantom: PhantomData<*const K>,
}

impl<K: ActionKind> Action<K> {
    /// Get the current state of the action.
    #[must_use]
    pub fn get(&self) -> K::Output {
        let input = self.session.input.read();
        let bindings_cache = self.session.bindings_cache.read();
        let enabled = self.set_enabled.load(Ordering::SeqCst);

        let bindings = if enabled {
            bindings_cache.get(&self.full_name).unwrap_or(&[])
        } else {
            &[]
        };

        let mut storage = self.storage.write();

        let inputs: Vec<_> = bindings
            .iter()
            .filter_map(|binding| storage.get(&input, binding))
            .collect();

        storage.reduce(&inputs)
    }

    /// Returns the full name of the action, including the action set it's part
    /// of.
    ///
    /// Action names are of the form `{set}/{action}`. An `Action` named "jump"
    /// created in an [`ActionSet`] named "gameplay" will have the name
    /// `gameplay/jump`.
    pub fn name(&self) -> &str {
        &self.full_name
    }
}

impl<K> Debug for Action<K>
where
    K: ActionKind,
    K::Output: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Action(name = {:?}, value = {:?})",
            self.name(),
            self.get()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{ActionSetBindings, Button, EventAction, KeyboardKey};

    use super::*;

    #[test]
    fn event_action() {
        let mut session = Session::new();
        let set = session.create_action_set("gameplay");
        let jump = set.create_action("jump", EventAction);

        let mut bindings = Bindings::new();
        let mut gameplay = ActionSetBindings::new();
        gameplay.insert("jump", vec![Button::Keyboard(KeyboardKey::Space).into()]);
        bindings.insert("gameplay", gameplay);

        session.use_bindings(bindings);

        assert!(!jump.get());

        session.handle_event(Event::ButtonPressed(KeyboardKey::Space.into()));
        assert!(jump.get());

        session.end_update();
        assert!(!jump.get());

        session.handle_event(Event::ButtonReleased(KeyboardKey::Space.into()));
        session.handle_event(Event::ButtonPressed(KeyboardKey::Space.into()));
        assert!(jump.get());
    }
}
