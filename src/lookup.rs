use std::collections::hash_map::{HashMap, RandomState};

#[derive(Debug, PartialEq)]
pub struct Lookup<T: Named> {
    map: HashMap<String, usize, RandomState>,
    items: Vec<T>,
}

impl<T: Named> Lookup<T> {
    pub fn new() -> Self {
        Lookup {
            map: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        let name = item.name().to_owned();
        self.items.push(item);
        self.map.insert(name, self.items.len() - 1);
    }

    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        match self.map.get(name) {
            Some(index) => { self.items.get(index.clone()) }
            None => { None },
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn swap_by_index(&mut self, i1: usize, i2: usize) -> Result<(), String> {
        // Avoid allocating the String unless there's actually an error
        fn invalid_index(i: usize) -> String {
            format!("Invalid index: {}", i)
        }

        let old_s1 = match self.items.get(i1) {
            Some(item) => { item.name().to_owned() },
            None => { return Err(invalid_index(i1)) },
        };

        let old_s2 = match self.items.get(i2) {
            Some(item) => { item.name().to_owned() },
            None => { return Err(invalid_index(i2)) },
        };

        let old_i1 = self.map.get(&old_s1).unwrap().clone();
        let old_i2 = self.map.get(&old_s2).unwrap().clone();

        self.map.insert(old_s1, old_i2);
        self.map.insert(old_s2, old_i1);
        self.items.swap(i1, i2);

        Ok(())
    }

    pub fn swap_by_name(&mut self, s1: &str, s2: &str) -> Result<(), String> {
        // Avoid allocating the String unless there's actually an error
        fn invalid_name(s: &str) -> String {
            format!("Invalid name: {}", s)
        }

        let old_i1 = match self.map.get(s1) {
            Some(index) => { index.clone() }
            None => { return Err(invalid_name(&s1)) },
        };

        let old_i2 = match self.map.get(s2) {
            Some(index) => { index.clone() }
            None => { return Err(invalid_name(&s1)) },
        };

        let old_s1 = self.items.get(old_i1).unwrap().name().to_owned();
        let old_s2 = self.items.get(old_i2).unwrap().name().to_owned();

        self.map.insert(old_s1, old_i2);
        self.map.insert(old_s2, old_i1);
        self.items.swap(old_i1, old_i2);

        Ok(())
    }
}

pub trait Named {
    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use lookup::*;

    #[derive(Debug, PartialEq)]
    struct Item {
        name: String,
    }

    impl Item {
        fn new(n: &str) -> Self {
            Item {
                name: n.to_owned(),
            }
        }
    }

    impl Named for Item {
        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn indexes() {
        let mut lookup = Lookup::new();
        lookup.push(Item::new("foo"));
        lookup.push(Item::new("bar"));

        let first_index = lookup.map.get("foo").unwrap().clone();
        let second_index = lookup.map.get("bar").unwrap().clone();
        assert_eq!(first_index, 0);
        assert_eq!(second_index, 1);
    }

    #[test]
    fn equality_with_two_items() {
        let mut first = Lookup::new();
        first.push(Item::new("foo"));
        first.push(Item::new("bar"));

        let mut second = Lookup::new();
        second.push(Item::new("foo"));
        second.push(Item::new("bar"));

        assert_eq!(first, second);
    }

    #[test]
    #[should_panic]
    fn equality_with_two_different_items() {
        let mut first = Lookup::new();
        first.push(Item::new("foo"));
        first.push(Item::new("bar"));

        let mut second = Lookup::new();
        second.push(Item::new("foo"));
        second.push(Item::new("quux"));

        assert_eq!(first, second);
    }

    #[test]
    fn swap_two_items_by_index() {
        let mut first = Lookup::new();
        first.push(Item::new("bar"));
        first.push(Item::new("foo"));

        let mut second = Lookup::new();
        second.push(Item::new("foo"));
        second.push(Item::new("bar"));

        first.swap_by_index(0, 1).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn swap_with_invalid_index() {
        let mut lookup = Lookup::new();
        lookup.push(Item::new("bar"));
        lookup.push(Item::new("foo"));

        assert!(lookup.swap_by_index(0, 2).is_err());
    }

    #[test]
    fn swap_two_items_by_name() {
        let mut first = Lookup::new();
        first.push(Item::new("bar"));
        first.push(Item::new("foo"));

        let mut second = Lookup::new();
        second.push(Item::new("foo"));
        second.push(Item::new("bar"));

        first.swap_by_name("foo", "bar").unwrap();
        assert_eq!(first, second);
    }
}
