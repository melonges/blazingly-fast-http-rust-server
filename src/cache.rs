use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::config::Config;

pub struct Cache {
    vector: Vec<Option<CacheItem>>,
}
pub struct CacheItem {
    pub key: String,
    pub value: String,
}

impl Cache {
    pub fn new(config: &Config) -> Cache {
        let mut v = Vec::new();
        v.resize_with(config.cache_size, || None);
        Cache { vector: v }
    }

    pub fn get(&self, key: &str) -> Option<&CacheItem> {
        let index = self.hash_key_to_index(key);
        let element = self.vector[index].as_ref();
        match element {
            Some(value) => {
                if value.key == *key {
                    Some(value)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> &String {
        let index = self.hash_key_to_index(&key);
        let cache_item = CacheItem { key, value };
        self.vector[index] = Some(cache_item);
        &self.vector[index].as_ref().unwrap().value
    }

    pub fn hash_key_to_index(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.vector.capacity()
    }
}
