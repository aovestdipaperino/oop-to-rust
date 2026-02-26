//! Chapter 13: Concurrency Foundations - Thread-Safe Cache

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct Cache<K, V> {
    data: RwLock<HashMap<K, V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().unwrap();
        data.get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
    }

    fn get_or_insert_with<F>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        // Try read first
        {
            let data = self.data.read().unwrap();
            if let Some(value) = data.get(&key) {
                return value.clone();
            }
        }

        // Need to compute and insert
        let mut data = self.data.write().unwrap();
        // Double-check after acquiring write lock
        if let Some(value) = data.get(&key) {
            return value.clone();
        }

        let value = f();
        data.insert(key, value.clone());
        value
    }

    fn len(&self) -> usize {
        let data = self.data.read().unwrap();
        data.len()
    }

    fn clear(&self) {
        let mut data = self.data.write().unwrap();
        data.clear();
    }
}

fn expensive_computation(n: u64) -> u64 {
    println!("  Computing fibonacci({})...", n);
    thread::sleep(Duration::from_millis(100));
    if n <= 1 {
        n
    } else {
        // Simplified iterative fibonacci
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 2..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

fn main() {
    println!("=== Thread-Safe Cache ===\n");

    let cache: Arc<Cache<u64, u64>> = Arc::new(Cache::new());
    let mut handles = vec![];

    // Multiple threads requesting the same keys
    let keys = vec![10, 20, 10, 30, 20, 10, 40, 30];

    for (i, key) in keys.into_iter().enumerate() {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let value = cache.get_or_insert_with(key, || expensive_computation(key));
            println!("Thread {}: fib({}) = {}", i, key, value);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nCache size: {}", cache.len());

    println!("\n=== Reading from populated cache ===\n");

    // These should be instant (cached)
    for key in [10, 20, 30, 40] {
        if let Some(value) = cache.get(&key) {
            println!("Cached fib({}) = {}", key, value);
        }
    }

    println!("\n=== Cache miss ===\n");
    let value = cache.get_or_insert_with(50, || expensive_computation(50));
    println!("fib(50) = {}", value);

    println!("\nFinal cache size: {}", cache.len());
}
