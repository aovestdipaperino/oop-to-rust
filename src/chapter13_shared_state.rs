//! Chapter 13: Concurrency Foundations - Shared State

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn demonstrate_arc_mutex() {
    println!("=== Arc<Mutex<T>> Counter ===\n");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("Thread {} incremented to {}", i, *num);
                drop(num); // Release lock before sleeping
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal count: {}", *counter.lock().unwrap());
}

fn demonstrate_rwlock() {
    println!("\n=== RwLock for Read-Heavy Workloads ===\n");

    #[derive(Debug)]
    struct Config {
        max_connections: u32,
        timeout_ms: u64,
    }

    let config = Arc::new(RwLock::new(Config {
        max_connections: 100,
        timeout_ms: 5000,
    }));

    let mut handles = vec![];

    // Spawn reader threads
    for i in 0..3 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            for _ in 0..3 {
                let cfg = config.read().unwrap();
                println!(
                    "Reader {}: max_connections={}, timeout={}ms",
                    i, cfg.max_connections, cfg.timeout_ms
                );
                drop(cfg);
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }

    // Spawn writer thread
    {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(15));
            let mut cfg = config.write().unwrap();
            println!("Writer: Updating config...");
            cfg.max_connections = 200;
            cfg.timeout_ms = 10000;
            println!("Writer: Config updated");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_config = config.read().unwrap();
    println!("\nFinal config: {:?}", *final_config);
}

fn demonstrate_deadlock_prevention() {
    println!("\n=== Deadlock Prevention with Lock Ordering ===\n");

    let resource_a = Arc::new(Mutex::new("Resource A"));
    let resource_b = Arc::new(Mutex::new("Resource B"));

    let ra1 = Arc::clone(&resource_a);
    let rb1 = Arc::clone(&resource_b);

    let ra2 = Arc::clone(&resource_a);
    let rb2 = Arc::clone(&resource_b);

    // Both threads acquire locks in the same order (A then B)
    let handle1 = thread::spawn(move || {
        let _a = ra1.lock().unwrap();
        println!("Thread 1: Got A");
        thread::sleep(Duration::from_millis(10));
        let _b = rb1.lock().unwrap();
        println!("Thread 1: Got B");
    });

    let handle2 = thread::spawn(move || {
        let _a = ra2.lock().unwrap();
        println!("Thread 2: Got A");
        thread::sleep(Duration::from_millis(10));
        let _b = rb2.lock().unwrap();
        println!("Thread 2: Got B");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("No deadlock occurred!");
}

fn main() {
    demonstrate_arc_mutex();
    demonstrate_rwlock();
    demonstrate_deadlock_prevention();
}
