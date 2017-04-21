use std::collections::hash_map::{HashMap, RandomState};

pub struct Lookup<T> {
    map: HashMap<String, usize, RandomState>,
    items: Vec<T>,
}

impl<T: Named> Lookup<T> {
    fn from_name(&self, name: &str) -> Option<&T> {
        match self.map.get(name) {
            Some(index) => { self.items.get(index.clone()) }
            None => { None },
        }
    }

    fn from_index(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn insert(&mut self, item: T) {
        let name = item.name().to_owned();
        self.items.push(item);
        self.map.insert(name, self.items.len());
    }
}

pub trait Named {
    fn name(&self) -> &str;
}
