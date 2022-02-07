use std::collections::HashMap;

use crate::bindings::Binding;

pub(super) struct BindingsCache {
    bindings: HashMap<String, Vec<Binding>>,
}

impl BindingsCache {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
    }

    pub fn get(&self, action_name: &str) -> Option<&[Binding]> {
        self.bindings.get(action_name).map(|vec| vec.as_slice())
    }

    pub fn insert(&mut self, action_name: String, bindings: Vec<Binding>) {
        self.bindings.insert(action_name, bindings);
    }
}
