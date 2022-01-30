use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::actions::ActionKind;
use crate::bindings::{Binding, Bindings};
use crate::state::InputState;
use crate::Event;

pub struct Session {
    inner: Arc<SessionInner>,
}

struct SessionInner {
    input: RwLock<InputState>,
    bindings: RwLock<Bindings>,
    bindings_cache: RwLock<BindingsCache>,
    next_action_id: AtomicUsize,
}

impl Session {
    /// Create a new Keybee Session.
    pub fn new() -> Self {
        let inner = Arc::new(SessionInner {
            input: RwLock::new(InputState::new()),
            bindings: RwLock::new(Bindings::empty()),
            bindings_cache: RwLock::new(BindingsCache::new()),
            next_action_id: AtomicUsize::new(0),
        });

        Self { inner }
    }

    /// Create a new action set with the given name.
    #[must_use]
    pub fn create_action_set(&self, name: &'static str) -> ActionSet {
        ActionSet {
            session: self.inner.clone(),
            enabled: Arc::new(AtomicBool::new(true)),
            name,
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

        for (set_name, action_set) in &bindings.action_sets {
            for (action_name, action) in &action_set.actions {
                let full_name = format!("{}/{}", set_name, action_name);

                if let Some(&index) = bindings_cache.action_name_to_index.get(&full_name) {
                    let add_cap = (index + 1).checked_sub(bindings_cache.bindings.len());

                    if let Some(add) = add_cap {
                        for _ in 0..add {
                            bindings_cache.bindings.push(Vec::new());
                        }
                    }

                    bindings_cache.bindings[index] = action.clone();
                }
            }
        }
    }

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
}

struct BindingsCache {
    action_name_to_index: HashMap<String, usize>,
    bindings: Vec<Vec<Binding>>,
}

impl BindingsCache {
    fn new() -> Self {
        Self {
            action_name_to_index: HashMap::new(),
            bindings: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.action_name_to_index.clear();
        self.bindings.clear();
    }
}

/// Defines a group of actions that a player can perform.
///
/// Created with [`Session::create_action_set`].
pub struct ActionSet {
    session: Arc<SessionInner>,
    enabled: Arc<AtomicBool>,
    name: &'static str,
}

impl ActionSet {
    /// Create a new action that can be activated by the player.
    #[must_use]
    pub fn create_action<K: ActionKind>(&self, name: &'static str) -> Action<K> {
        let id = self.session.next_action_id.fetch_add(1, Ordering::SeqCst);
        let full_name = format!("{}/{}", self.name, name);

        Action {
            session: self.session.clone(),
            set_enabled: Arc::clone(&self.enabled),
            id,
            full_name,
            _phantom: PhantomData,
        }
    }

    /// Enable or disable all actions within this action set.
    pub fn set_enabled(&self, value: bool) {
        self.enabled.store(value, Ordering::SeqCst);
    }
}

/// Defines something that a player can do.
///
/// Created with [`ActionSet::create_action`].
pub struct Action<K> {
    session: Arc<SessionInner>,
    set_enabled: Arc<AtomicBool>,
    id: usize,
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
            bindings_cache
                .bindings
                .get(self.id)
                .map(Vec::as_slice)
                .unwrap_or(&[])
        } else {
            &[]
        };

        let inputs: Vec<_> = bindings
            .iter()
            .filter_map(|binding| K::get(&input, binding))
            .collect();

        K::reduce(&inputs)
    }

    /// Returns the full name of the action, including the set it's part of.
    pub fn name(&self) -> &str {
        &self.full_name
    }
}
