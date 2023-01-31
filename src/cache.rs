use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::config::Config;

pub struct CacheItem {
    pub key: String,
    pub value: String,
    pub prev: Option<Box<CacheItem>>,
    pub next: Option<Box<CacheItem>>,
}
pub struct LRUCache {
    left: Box<CacheItem>,
    right: Box<CacheItem>,
    capacity: usize,
    cache: HashMap<String, Box<CacheItem>>,
}

impl CacheItem {
    fn new(key: String, value: String) -> CacheItem {
        CacheItem {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

impl LRUCache {
    pub fn new(config: &Config) -> LRUCache {
        let key = "0".to_string();
        let value = "0".to_string();
        let mut left = Box::new(CacheItem::new(key, value));
        let mut right = Box::new(CacheItem::new(key, value));
        left.next = Some(right);
        right.prev = Some(left);
        LRUCache {
            left,
            right,
            capacity: config.cache_size,
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(v) = self.cache.get(key) {
            self.remove(*v);
            self.insert(*v);
            return Some(v.value);
        }
        None
    }

    pub fn put(&self, key: String, value: String) {
        if self.cache.get(&key).is_some() {
            self.cache.remove(&key);
        }
        let cache_item = Box::new(CacheItem::new(key, value));
        self.cache.insert(key, cache_item);
        self.insert(cache_item);

        if self.cache.len() > self.capacity {
            let lru = self.left.next.unwrap();
            self.remove(lru);
            self.cache.remove(&lru.key);
        }
    }

    fn remove(&self, cache_item: Box<CacheItem>) {
        let prev = cache_item.prev.unwrap();
        let next = cache_item.next.unwrap();
        prev.next = Some(next);
        next.prev = Some(prev);
    }

    fn insert(&self, cache_item: Box<CacheItem>) {
        let prev = self.right.prev.unwrap();
        let next = self.right;
        prev.next = Some(cache_item);
        next.prev = Some(cache_item);
        cache_item.prev = Some(prev);
        cache_item.next = Some(next);
    }
}
