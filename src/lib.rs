use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Entry<'a, K, V> {
    map: HashMap<&'a K, &'a V>,
    idx: usize,
}

impl<'a, K, V> fmt::Display for Entry<'a, K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Entry")
            .field("map", &self.map)
            .field("idx", &self.idx)
            .finish()
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::fmt::Display,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            idx: 0,
        }
    }
}

// Randomly selects a candidate item and discards it to make space when necessary.
// This algorithm does not require keeping any information about the access history.
#[derive(Debug)]
pub struct RRCache<'a, K, V> {
    entry_map: HashMap<&'a K, Entry<'a, K, V>>,
    keys: Vec<&'a K>,
}

impl<'a, K, V> fmt::Display for RRCache<'a, K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RRCache")
            .field("entry_map", &self.entry_map)
            .field("keys", &self.keys)
            .finish()
    }
}

impl<'a, K, V> RRCache<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::fmt::Display,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            entry_map: HashMap::with_capacity(capacity),
            keys: Vec::new(),
        }
    }

    // Time: O(1) | Space: O(n)
    pub fn set(&mut self, key: &'a K, value: &'a V) -> bool {
        if let Some(entry) = self.entry_map.get_mut(key) {
            entry.map.insert(key, value);
            return true;
        }
        if self.entry_map.len() == self.entry_map.capacity() {
            let mut rng = thread_rng();
            let rand_key = match self.keys.choose(&mut rng) {
                Some(k) => *k,
                None => return false,
            };
            let rand_entry = match self.entry_map.get_key_value(rand_key) {
                Some((_, entry)) => entry,
                None => return false,
            };

            let last_idx = self.keys.len() - 1;
            self.keys.swap(rand_entry.idx, last_idx);
            self.keys.pop();
            self.entry_map.remove(rand_key);
        }
        self.keys.push(key);
        let mut entry = Entry::new(self.entry_map.capacity());
        entry.map.insert(key, value);
        entry.idx = self.keys.len() - 1;
        self.entry_map.insert(key, entry);
        true
    }

    // Time: O(1) | Space: O(1)
    pub fn get(&mut self, key: &'a K) -> Option<&V> {
        let entry = match self.entry_map.get(key) {
            Some(entry) => entry,
            None => return None,
        };
        entry.map.get(key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rr_cache() {
        let mut rr_cache = RRCache::new(3);
        assert_eq!(rr_cache.get(&1), None);
        assert_eq!(rr_cache.set(&1, &"one"), true);
        assert_eq!(rr_cache.get(&1), Some(&"one"));

        assert_eq!(rr_cache.set(&2, &"two"), true);
        assert_eq!(rr_cache.set(&3, &"three"), true);
        assert_eq!(rr_cache.set(&4, &"four"), true);
        assert!(rr_cache.get(&2).is_some());
    }
}
