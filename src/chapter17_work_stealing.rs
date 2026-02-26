//! Chapter 17: Concurrent Data Structures - Work Stealing

use crossbeam::deque::{Injector, Stealer, Worker};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn demonstrate_work_stealing() {
    println!("=== Work-Stealing Deque ===\n");

    // Global injector for submitting work
    let injector: Arc<Injector<u64>> = Arc::new(Injector::new());

    // Create workers and collect their stealers
    let num_workers = 4;
    let mut workers = Vec::new();
    let mut stealers = Vec::new();

    for _ in 0..num_workers {
        let worker = Worker::new_fifo();
        stealers.push(worker.stealer());
        workers.push(worker);
    }

    let stealers: Arc<Vec<Stealer<u64>>> = Arc::new(stealers);
    let running = Arc::new(AtomicBool::new(true));

    // Spawn worker threads
    let mut handles = vec![];

    for (id, worker) in workers.into_iter().enumerate() {
        let injector = Arc::clone(&injector);
        let stealers = Arc::clone(&stealers);
        let running = Arc::clone(&running);

        handles.push(thread::spawn(move || {
            let mut processed = 0u64;
            let mut stolen = 0u64;

            while running.load(Ordering::Relaxed) || !worker.is_empty() {
                // First try local queue
                if let Some(task) = worker.pop() {
                    // Process task
                    processed += 1;
                    thread::sleep(Duration::from_micros(task * 10));
                    continue;
                }

                // Try global injector
                if let crossbeam::deque::Steal::Success(task) = injector.steal() {
                    processed += 1;
                    thread::sleep(Duration::from_micros(task * 10));
                    continue;
                }

                // Try stealing from other workers
                for (i, stealer) in stealers.iter().enumerate() {
                    if i != id {
                        if let crossbeam::deque::Steal::Success(task) = stealer.steal() {
                            stolen += 1;
                            processed += 1;
                            thread::sleep(Duration::from_micros(task * 10));
                            break;
                        }
                    }
                }

                // Small sleep to avoid busy-waiting
                thread::sleep(Duration::from_micros(100));
            }

            println!(
                "Worker {}: processed {} tasks ({} stolen)",
                id, processed, stolen
            );
            processed
        }));
    }

    // Submit work to the global injector
    println!("Submitting 100 tasks...\n");
    for i in 0..100 {
        injector.push(i % 10 + 1); // Tasks with varying "costs"
    }

    // Wait for work to be processed
    thread::sleep(Duration::from_millis(500));
    running.store(false, Ordering::Relaxed);

    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("\nTotal tasks processed: {}", total);
}

fn demonstrate_dashmap() {
    println!("\n=== DashMap (Concurrent HashMap) ===\n");

    use dashmap::DashMap;

    let map: Arc<DashMap<String, u64>> = Arc::new(DashMap::new());
    let mut handles = vec![];

    // Writers
    for i in 0..4 {
        let map = Arc::clone(&map);
        handles.push(thread::spawn(move || {
            for j in 0..25 {
                let key = format!("key_{}", j);
                map.entry(key)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
            println!("Writer {} finished", i);
        }));
    }

    // Readers (concurrent with writers)
    for i in 0..2 {
        let map = Arc::clone(&map);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            let mut sum = 0;
            for entry in map.iter() {
                sum += *entry.value();
            }
            println!("Reader {} saw sum: {}", i, sum);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal map size: {}", map.len());

    // Show some values
    println!("\nSample values:");
    for i in 0..5 {
        let key = format!("key_{}", i);
        if let Some(value) = map.get(&key) {
            println!("  {} = {}", key, *value);
        }
    }
}

fn demonstrate_crossbeam_channel() {
    println!("\n=== Crossbeam MPMC Channel ===\n");

    use crossbeam::channel;

    let (tx, rx) = channel::bounded::<u64>(10);

    let mut handles = vec![];

    // Multiple producers
    for i in 0..3 {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            for j in 0..10 {
                let value = i * 100 + j;
                tx.send(value).unwrap();
                println!("Producer {} sent {}", i, value);
            }
        }));
    }

    drop(tx); // Drop original sender

    // Multiple consumers
    for i in 0..2 {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            let mut count = 0;
            while let Ok(value) = rx.recv() {
                println!("Consumer {} received {}", i, value);
                count += 1;
            }
            println!("Consumer {} processed {} items", i, count);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    demonstrate_work_stealing();
    demonstrate_dashmap();
    demonstrate_crossbeam_channel();

    println!("\n=== All concurrent data structure demos completed ===");
}
