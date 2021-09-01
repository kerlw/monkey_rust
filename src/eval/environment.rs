use crate::eval::ObjectWrapper;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Environment<'a> {
    identifiers: HashMap<&'a str, ObjectWrapper>,
}

impl<'a, 'b> Environment<'a> {
    pub fn contains(&self, name: &str) -> bool {
        self.identifiers.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&ObjectWrapper> {
        self.identifiers.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ObjectWrapper> {
        self.identifiers.get_mut(name)
    }

    pub fn set(&mut self, name: &'b str, obj: ObjectWrapper) -> Option<ObjectWrapper> where 'b: 'a{
        self.identifiers.insert(name, obj)
    }
}
