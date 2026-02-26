//! Chapter 17: Concurrent Data Structures - Atomics

use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn demonstrate_atomic_counter() {
    println!("=== Atomic Counter ===\n");

    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    for i in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
            println!("Thread {} finished", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.load(Ordering::Relaxed));
    println!("Expected: 4000");
}

fn demonstrate_atomic_flag() {
    println!("\n=== Atomic Flag for Shutdown ===\n");

    let running = Arc::new(AtomicBool::new(true));
    let mut handles = vec![];

    for i in 0..3 {
        let running = Arc::clone(&running);
        handles.push(thread::spawn(move || {
            let mut iterations = 0;
            while running.load(Ordering::Relaxed) {
                iterations += 1;
                thread::sleep(Duration::from_millis(10));
            }
            println!("Worker {} did {} iterations", i, iterations);
        }));
    }

    thread::sleep(Duration::from_millis(100));
    println!("Setting shutdown flag...");
    running.store(false, Ordering::Relaxed);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All workers stopped");
}

fn demonstrate_compare_and_swap() {
    println!("\n=== Compare-and-Swap ===\n");

    let value = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    // Multiple threads try to increment, but only the first wins
    for i in 0..5 {
        let value = Arc::clone(&value);
        handles.push(thread::spawn(move || {
            // Try to change 0 to 1
            match value.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(old) => println!("Thread {}: Successfully changed {} to 1", i, old),
                Err(current) => println!("Thread {}: Failed, current value is {}", i, current),
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", value.load(Ordering::SeqCst));
}

fn demonstrate_lock_free_max() {
    println!("\n=== Lock-Free Maximum ===\n");

    let max_value = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    // Each thread proposes a maximum value
    let proposed_values = vec![42, 100, 7, 256, 50, 128];

    for value in proposed_values {
        let max_value = Arc::clone(&max_value);
        handles.push(thread::spawn(move || {
            loop {
                let current = max_value.load(Ordering::Relaxed);
                if value <= current {
                    println!("Thread proposing {}: {} is not greater than current {}", value, value, current);
                    break;
                }
                match max_value.compare_exchange_weak(
                    current,
                    value,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        println!("Thread proposing {}: Updated max to {}", value, value);
                        break;
                    }
                    Err(_) => continue, // Retry
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal maximum: {}", max_value.load(Ordering::Relaxed));
}

fn demonstrate_statistics_counter() {
    println!("\n=== Lock-Free Statistics ===\n");

    struct Stats {
        count: AtomicU64,
        sum: AtomicU64,
    }

    impl Stats {
        fn new() -> Self {
            Self {
                count: AtomicU64::new(0),
                sum: AtomicU64::new(0),
            }
        }

        fn record(&self, value: u64) {
            self.count.fetch_add(1, Ordering::Relaxed);
            self.sum.fetch_add(value, Ordering::Relaxed);
        }

        fn average(&self) -> f64 {
            let count = self.count.load(Ordering::Relaxed);
            let sum = self.sum.load(Ordering::Relaxed);
            if count == 0 {
                0.0
            } else {
                sum as f64 / count as f64
            }
        }
    }

    let stats = Arc::new(Stats::new());
    let mut handles = vec![];

    for i in 0..4 {
        let stats = Arc::clone(&stats);
        handles.push(thread::spawn(move || {
            for j in 0..100 {
                stats.record((i * 100 + j) as u64);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Count: {}", stats.count.load(Ordering::Relaxed));
    println!("Sum: {}", stats.sum.load(Ordering::Relaxed));
    println!("Average: {:.2}", stats.average());
}

fn main() {
    demonstrate_atomic_counter();
    demonstrate_atomic_flag();
    demonstrate_compare_and_swap();
    demonstrate_lock_free_max();
    demonstrate_statistics_counter();
}
