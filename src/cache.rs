use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::system_utils::read_files;
use crate::{status_headers::StatusHeader, DEFAULT_CACHE_SIZE};
struct Cache {
    vector: Vec<String>,
}

impl Cache {
    fn new(size: usize) -> Cache {
        Cache {
            vector: Vec::with_capacity(size),
        }
    }

    fn get(&mut self, key: String) ->  {}

    fn hash_to_index(key: &String) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() & (DEFAULT_CACHE_SIZE - 1)) as usize
    }
}
