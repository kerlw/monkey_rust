use crate::eval::ObjectWrapper;
use std::collections::HashMap;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Environment {
    identifiers: HashMap<String, ObjectWrapper>,
}

impl Environment {
    pub fn contains(&self, name: &str) -> bool {
        self.identifiers.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&ObjectWrapper> {
        self.identifiers.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ObjectWrapper> {
        self.identifiers.get_mut(name)
    }

    pub fn set(&mut self, name: &str, obj: ObjectWrapper) -> Option<ObjectWrapper> {
        self.identifiers.insert(name.to_string(), obj)
    }
}
