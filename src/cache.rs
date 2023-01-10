use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

struct Cache<'a> {
    map: Vec<&'a String>,
}

impl Cache<'_> {
    fn new(size: usize) -> Cache<'static> {
        Cache {
            map: Vec::with_capacity(size),
        }
    }

    fn get(&mut self, key: String) {
        let index = Cache::hash_to_index(&key);
        let value = self.map[index];
    }

    fn hash_to_index(key: &String) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}
