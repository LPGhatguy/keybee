use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::actions::ActionKind;
use crate::bindings::{Binding, Bindings};
use crate::state::InputState;

pub struct Session {
    inner: Arc<SessionInner>,
}

struct SessionInner {
    input: RwLock<InputState>,
    bindings: RwLock<Bindings>,
    bindings_cache: RwLock<BindingsCache>,
    next_action_id: AtomicUsize,
}

#[derive(Default)]
struct BindingsCache {
    action_name_to_index: HashMap<String, usize>,
    bindings: Vec<Vec<Binding>>,
}

impl Session {
    pub fn new() -> Self {
        let inner = Arc::new(SessionInner {
            input: RwLock::new(InputState::new()),
            bindings: RwLock::new(Bindings::empty()),
            bindings_cache: RwLock::new(BindingsCache::default()),
            next_action_id: AtomicUsize::new(0),
        });

        Self { inner }
    }

    pub fn create_action_set(&self, name: &'static str) -> ActionSet {
        ActionSet {
            session: self.inner.clone(),
            enabled: Arc::new(AtomicBool::new(true)),
            name,
        }
    }

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

    #[cfg(feature = "gilrs")]
    pub fn handle_gilrs_event(&mut self, event: &gilrs::Event) {
        let mut input = self.inner.input.write();
        input.handle_gilrs_event(event);
    }

    #[cfg(feature = "winit")]
    pub fn handle_winit_event<T>(&mut self, event: &winit::event::Event<T>) {
        let mut input = self.inner.input.write();
        input.handle_winit_event(event);
    }

    pub fn end_update(&mut self) {
        let mut input = self.inner.input.write();
        input.end_update();
    }
}

pub struct ActionSet {
    session: Arc<SessionInner>,
    enabled: Arc<AtomicBool>,
    name: &'static str,
}

impl ActionSet {
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

    pub fn set_enabled(&self, value: bool) {
        self.enabled.store(value, Ordering::SeqCst);
    }
}

pub struct Action<K> {
    session: Arc<SessionInner>,
    set_enabled: Arc<AtomicBool>,
    id: usize,
    full_name: String,
    _phantom: PhantomData<*const K>,
}

impl<K: ActionKind> Action<K> {
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

    pub fn name(&self) -> &str {
        &self.full_name
    }
}
