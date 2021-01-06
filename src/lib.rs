//! An in-memory key-value store

#![deny(missing_docs)]
use std::collections::HashMap;

/// Contains a HashMap that serves as the store for the key-value library
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>
}

impl KvStore {
// impl Default for KvStore {
    /// This method allows the creation of an in-memory key-value store
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        let store = HashMap::new();
        KvStore {
            store
        }
    }

    /// This method stores a value associated with a key
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// ``` 
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// This method returns a value associated with a key
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// let value = store.get("key1".to_string());
    /// ``` 
    pub fn get(&self, key: String) -> Option<String> {
        match self.store.get(&key) {
            Some(value) => Some(value.to_string()),
            None => None
        }
    }

    /// This method deletes a value associated with a key
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// let value = store.remove("key1".to_string());
    /// ``` 
    pub fn remove(&mut self, key: String) {
        //panic!();
        self.store.remove(&key);
    }
}
