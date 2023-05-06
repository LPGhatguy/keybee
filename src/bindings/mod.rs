mod binding;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use binding::*;

/// Defines how inputs should be mapped to actions.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bindings {
    pub action_sets: HashMap<String, ActionSetBindings>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            action_sets: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&ActionSetBindings> {
        self.action_sets.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ActionSetBindings> {
        self.action_sets.get_mut(name)
    }

    pub fn insert<S: Into<String>>(&mut self, name: S, value: ActionSetBindings) {
        self.action_sets.insert(name.into(), value);
    }

    /// Merge another set of bindings into this one, overwriting any actions
    /// defined in `other`.
    ///
    /// This method is useful for applying user-specified bindings over the top
    /// of default bindings.
    pub fn merge(&mut self, other: Bindings) {
        for (name, action_set) in other.action_sets {
            let bindings = self
                .action_sets
                .entry(name)
                .or_insert_with(ActionSetBindings::new);
            bindings.merge(action_set);
        }
    }

    pub fn clear(&mut self) {
        self.action_sets.clear();
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActionSetBindings {
    pub actions: HashMap<String, Vec<Binding>>,
}

impl ActionSetBindings {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<Binding>> {
        self.actions.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Vec<Binding>> {
        self.actions.get_mut(name)
    }

    pub fn insert<S: Into<String>>(&mut self, name: S, value: Vec<Binding>) {
        self.actions.insert(name.into(), value);
    }

    pub fn merge(&mut self, other: ActionSetBindings) {
        for (name, action) in other.actions {
            self.actions.insert(name, action);
        }
    }
}
