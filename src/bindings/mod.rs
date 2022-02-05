mod binding;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use binding::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bindings {
    pub action_sets: HashMap<String, ActionSetBindings>,
}

impl Bindings {
    pub fn empty() -> Self {
        Self {
            action_sets: HashMap::new(),
        }
    }

    pub fn new<A, Name>(action_sets: A) -> Self
    where
        A: IntoIterator<Item = (Name, ActionSetBindings)>,
        Name: Into<String>,
    {
        let action_sets = action_sets
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect();

        Self { action_sets }
    }

    pub fn merge(&mut self, other: Bindings) {
        for (name, action_set) in other.action_sets {
            let bindings = self
                .action_sets
                .entry(name)
                .or_insert_with(ActionSetBindings::empty);
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
    pub fn empty() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn new<A, N, Bs, B>(actions: A) -> Self
    where
        A: IntoIterator<Item = (N, Bs)>,
        N: Into<String>,
        Bs: IntoIterator<Item = B>,
        B: Into<Binding>,
    {
        let actions = actions
            .into_iter()
            .map(|(key, value)| (key.into(), value.into_iter().map(Into::into).collect()))
            .collect();

        Self { actions }
    }

    pub fn merge(&mut self, other: ActionSetBindings) {
        for (name, action) in other.actions {
            self.actions.insert(name, action);
        }
    }
}
