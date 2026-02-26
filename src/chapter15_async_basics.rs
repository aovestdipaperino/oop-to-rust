//! Chapter 15: Async Rust - Basics

use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

async fn fetch_data(id: u32) -> String {
    println!("[Task {}] Starting fetch...", id);
    sleep(Duration::from_millis(100 * id as u64)).await;
    println!("[Task {}] Fetch complete", id);
    format!("Data from task {}", id)
}

async fn demonstrate_concurrent_tasks() {
    println!("=== Concurrent Tasks with join! ===\n");

    let (r1, r2, r3) = tokio::join!(fetch_data(1), fetch_data(2), fetch_data(3));

    println!("\nResults:");
    println!("  {}", r1);
    println!("  {}", r2);
    println!("  {}", r3);
}

async fn demonstrate_spawned_tasks() {
    println!("\n=== Spawned Tasks ===\n");

    let mut handles = vec![];

    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(50 * i)).await;
            format!("Task {} completed", i)
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Task failed: {}", e),
        }
    }
}

async fn demonstrate_async_channels() {
    println!("\n=== Async Channels ===\n");

    let (tx, mut rx) = mpsc::channel::<String>(10);

    // Producer task
    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            let msg = format!("Message {}", i);
            println!("Sending: {}", msg);
            tx.send(msg).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
    });

    // Consumer task
    let consumer = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Received: {}", msg);
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();
}

async fn demonstrate_select() {
    println!("\n=== Select (Racing Futures) ===\n");

    let slow = async {
        sleep(Duration::from_millis(200)).await;
        "slow result"
    };

    let fast = async {
        sleep(Duration::from_millis(50)).await;
        "fast result"
    };

    tokio::select! {
        result = slow => println!("Slow finished first: {}", result),
        result = fast => println!("Fast finished first: {}", result),
    }
}

async fn demonstrate_timeout() {
    println!("\n=== Timeout ===\n");

    let slow_operation = async {
        sleep(Duration::from_millis(500)).await;
        "completed"
    };

    match tokio::time::timeout(Duration::from_millis(100), slow_operation).await {
        Ok(result) => println!("Operation completed: {}", result),
        Err(_) => println!("Operation timed out!"),
    }

    let fast_operation = async {
        sleep(Duration::from_millis(50)).await;
        "completed"
    };

    match tokio::time::timeout(Duration::from_millis(100), fast_operation).await {
        Ok(result) => println!("Operation completed: {}", result),
        Err(_) => println!("Operation timed out!"),
    }
}

#[tokio::main]
async fn main() {
    demonstrate_concurrent_tasks().await;
    demonstrate_spawned_tasks().await;
    demonstrate_async_channels().await;
    demonstrate_select().await;
    demonstrate_timeout().await;

    println!("\n=== All async demos completed ===");
}
