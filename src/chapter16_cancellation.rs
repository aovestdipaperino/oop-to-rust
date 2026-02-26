//! Chapter 16: Cancellation and Graceful Shutdown

use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

async fn worker_with_token(id: u32, token: CancellationToken) {
    println!("[Worker {}] Started", id);

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("[Worker {}] Received cancellation signal", id);
                break;
            }
            _ = sleep(Duration::from_millis(200)) => {
                println!("[Worker {}] Working...", id);
            }
        }
    }

    println!("[Worker {}] Cleaning up...", id);
    sleep(Duration::from_millis(50)).await;
    println!("[Worker {}] Stopped", id);
}

async fn demonstrate_cancellation_token() {
    println!("=== CancellationToken ===\n");

    let token = CancellationToken::new();
    let mut handles = vec![];

    // Spawn workers
    for i in 1..=3 {
        let worker_token = token.clone();
        handles.push(tokio::spawn(async move {
            worker_with_token(i, worker_token).await;
        }));
    }

    // Let workers run for a bit
    sleep(Duration::from_millis(500)).await;

    // Cancel all workers
    println!("\n--- Cancelling all workers ---\n");
    token.cancel();

    // Wait for all workers to finish
    for handle in handles {
        handle.await.unwrap();
    }

    println!("\nAll workers stopped");
}

async fn worker_with_broadcast(id: u32, mut shutdown_rx: broadcast::Receiver<()>) {
    println!("[Worker {}] Started", id);

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                println!("[Worker {}] Shutdown signal received", id);
                break;
            }
            _ = sleep(Duration::from_millis(200)) => {
                println!("[Worker {}] Processing...", id);
            }
        }
    }

    println!("[Worker {}] Stopped", id);
}

async fn demonstrate_broadcast_shutdown() {
    println!("\n=== Broadcast Shutdown Signal ===\n");

    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let mut handles = vec![];

    for i in 1..=3 {
        let rx = shutdown_tx.subscribe();
        handles.push(tokio::spawn(async move {
            worker_with_broadcast(i, rx).await;
        }));
    }

    sleep(Duration::from_millis(500)).await;

    println!("\n--- Broadcasting shutdown ---\n");
    let _ = shutdown_tx.send(());

    for handle in handles {
        handle.await.unwrap();
    }

    println!("\nAll workers stopped");
}

struct GracefulShutdown {
    notify: broadcast::Sender<()>,
    complete_tx: mpsc::Sender<()>,
    complete_rx: mpsc::Receiver<()>,
}

impl GracefulShutdown {
    fn new() -> Self {
        let (notify, _) = broadcast::channel(1);
        let (complete_tx, complete_rx) = mpsc::channel(1);
        Self {
            notify,
            complete_tx,
            complete_rx,
        }
    }

    fn subscribe(&self) -> (broadcast::Receiver<()>, mpsc::Sender<()>) {
        (self.notify.subscribe(), self.complete_tx.clone())
    }

    fn trigger(&self) {
        let _ = self.notify.send(());
    }

    async fn wait_for_completion(&mut self, timeout: Duration) {
        drop(self.complete_tx.clone()); // Drop our copy

        match tokio::time::timeout(timeout, self.complete_rx.recv()).await {
            Ok(_) => println!("All tasks completed gracefully"),
            Err(_) => println!("Timeout waiting for tasks"),
        }
    }
}

async fn graceful_worker(id: u32, mut shutdown: broadcast::Receiver<()>, _done: mpsc::Sender<()>) {
    println!("[Worker {}] Started", id);

    loop {
        tokio::select! {
            _ = shutdown.recv() => {
                println!("[Worker {}] Shutting down gracefully...", id);
                // Simulate cleanup
                sleep(Duration::from_millis(100)).await;
                break;
            }
            _ = sleep(Duration::from_millis(150)) => {
                println!("[Worker {}] Working...", id);
            }
        }
    }

    println!("[Worker {}] Cleanup complete", id);
    // _done is dropped here, signaling completion
}

async fn demonstrate_graceful_shutdown() {
    println!("\n=== Graceful Shutdown Coordinator ===\n");

    let mut shutdown = GracefulShutdown::new();

    for i in 1..=3 {
        let (shutdown_rx, done_tx) = shutdown.subscribe();
        tokio::spawn(async move {
            graceful_worker(i, shutdown_rx, done_tx).await;
        });
    }

    sleep(Duration::from_millis(400)).await;

    println!("\n--- Initiating graceful shutdown ---\n");
    shutdown.trigger();

    shutdown.wait_for_completion(Duration::from_secs(5)).await;
}

#[tokio::main]
async fn main() {
    demonstrate_cancellation_token().await;
    demonstrate_broadcast_shutdown().await;
    demonstrate_graceful_shutdown().await;

    println!("\n=== All shutdown demos completed ===");
}
