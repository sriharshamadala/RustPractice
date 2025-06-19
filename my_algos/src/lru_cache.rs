//! # LRU Cache Implementation without LinkedList
//! 
//! ## Intuition
//! 
//! I am using a hashmap to store the key-value pairs, 
//! a Deque to store the keys based on the access times 
//! and another hashmap to store a list of keys to prune. 
//! This map also stores how many times a key must be pruned from the Deque.
//! Note that we are not using a linkedlist here.
//! 
//! ## Approach
//! 
//! My goal is avoid using a Linkedlist. While it has great 
//! theoretical properties, it isn't laid out well in memory 
//! and cannot benefit from cache locality. But this choice means 
//! I have to figure out how to keep the keys in order of 
//! their access times. I chose to violate the memory constraints 
//! temporarily to accomplish this. My Deque will likely contain 
//! many more keys than `capacity`. To be precise, there will be 
//! `len(cache) + num_gets + num_updates (puts)`. Here `num_gets` 
//! is the number of times `get()` is called since the last prune. 
//! But the key insight here is that you can call prune often so 
//! that this number is in the same order of `capacity`. I would 
//! need to call `prune()` from a low priority thread and would 
//! need synchornization now that it is a multithreaded design. 
//! 
//! ## Time Complexity
//! 
//!`get() - O(1)`
//!`put() - O(1)` -> If pruning is done in the background
//!`put() - O(#gets + #puts + capacity)` -> If pruning is done 
//! based on some specific event like reaching capacity or 
//! insert into cache. When amortized over the number of gets and puts, this is also `O(1)`.
//! 
//! ## Space complexity:
//! `O(capacity)` -> If pruning is done in the background. 
//! Otherwise the space required is unbounded, say if we 
//! only have `get()` calls and pruning is only done during
//! an insert, our `deque` will continue to grow.

use std::collections::{VecDeque, HashMap};

struct LRUCache {
    cache: HashMap<i32, i32>,
    keys_ordered_lru: VecDeque<i32>,
    keys_to_prune: HashMap<i32, i32>,
    capacity: usize
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {
            cache: HashMap::with_capacity(capacity as usize),
            keys_ordered_lru: VecDeque::with_capacity(capacity as usize),
            keys_to_prune: HashMap::with_capacity(capacity as usize),
            capacity: capacity as usize
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        // Check if our cache contains this key
        if let Some(&value) = self.cache.get(&key) {
            self.update_key_order(key);
            return value;
        }
        else {
            return -1;
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(v) = self.cache.get_mut(&key) {
            *v = value;
            self.update_key_order(key);
        }
        else {
            // insertion might result in exceeding capacity
            // Free up space if required
            self.prune();
            if self.cache.len() >= self.capacity {
                let key_popped = self.keys_ordered_lru.pop_front().unwrap();
                self.cache.remove(&key_popped);
            }

            // Now insert
            self.cache.insert(key, value);
            self.keys_ordered_lru.push_back(key);            
        }
    }

    // Remove the duplicated keys in our ordering
    // can be run in the background when system is idle or periodically
    // has to be called before popping elements out our cache for capacity constraints
    fn prune(&mut self) {
        // Take ownership of keys_ordered_lru, leaving an empty VecDeque in its place
        let original = std::mem::take(&mut self.keys_ordered_lru);
        
        // Now we can use into_iter() since we own original
        self.keys_ordered_lru = original
            .into_iter()
            .filter(|key| {
                if let Some(count) = self.keys_to_prune.get_mut(key) {
                    *count -= 1;
                    if *count == 0 {
                        self.keys_to_prune.remove(key);
                    }
                    false // Filter out this key
                } else {
                    true // Keep this key
                }
            })
            .collect();
    }

    /// Move the key to the back, indicating it is the most recently used
    fn update_key_order(&mut self, key: i32) {
        self.keys_ordered_lru.push_back(key);

        // since the key is duplicated we need to add it to the list to be pruned
        self.keys_to_prune.entry(key).and_modify(|counter| *counter += 1).or_insert(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_init() {
        let cache = LRUCache::new(2);
        assert_eq!(cache.capacity, 2);
        assert_eq!(cache.cache.len(), 0);
        assert_eq!(cache.keys_ordered_lru.len(), 0);
        assert_eq!(cache.keys_to_prune.len(), 0);   
    }

    #[test]
    fn test_lru_cache_put_and_get() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1); // returns 1
        cache.put(2, 2);
        assert_eq!(cache.get(2), 2); // returns 2
        cache.put(3, 3); // evicts key 1
        assert_eq!(cache.get(1), -1); // returns -1 (not found)
        assert_eq!(cache.get(2), 2); // returns 2
        assert_eq!(cache.get(3), 3); // returns 3
        cache.put(4, 4); // evicts key 2
        assert_eq!(cache.get(2), -1); // returns -1 (not found)
        assert_eq!(cache.get(3), 3); // returns 3
        assert_eq!(cache.get(4), 4); // returns 4
    }
}