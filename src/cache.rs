use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::status_headers::StatusHeader;
use crate::system_utils::read_files;
struct Cache {
    map: Vec<String>,
}

impl Cache {
    fn new(size: usize) -> Cache {
        Cache {
            map: Vec::with_capacity(size),
        }
    }

    fn get(&mut self, key: String) -> (StatusHeader, &String) {
        let index = Cache::hash_to_index(&key);
        let result = self.map.get(index);
        match result {
            Some(value) => (StatusHeader::Ok, value),
            None => {
                let resource = read_files(&key);
                self.map.insert(index, resource.1);
                (resource.0, self.map.get(index).unwrap())
            }
        }
    }

    fn hash_to_index(key: &String) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % 1000) as usize
    }
}
