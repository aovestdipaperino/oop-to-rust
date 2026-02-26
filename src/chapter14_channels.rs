//! Chapter 14: Message Passing - Basic Channels

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn basic_channel() {
    println!("=== Basic mpsc Channel ===\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}

fn multiple_producers() {
    println!("\n=== Multiple Producers ===\n");

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..3 {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("Message {} from producer {}", j, i);
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        }));
    }

    drop(tx); // Drop original sender

    for handle in handles {
        handle.join().unwrap();
    }

    for received in rx {
        println!("Received: {}", received);
    }
}

fn sync_channel() {
    println!("\n=== Synchronous (Bounded) Channel ===\n");

    let (tx, rx) = mpsc::sync_channel(2); // Buffer size of 2

    let producer = thread::spawn(move || {
        for i in 0..5 {
            println!("Sending {}...", i);
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    thread::sleep(Duration::from_millis(500));

    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
            thread::sleep(Duration::from_millis(200));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

fn request_response() {
    println!("\n=== Request-Response Pattern ===\n");

    #[derive(Debug)]
    struct Request {
        id: u32,
        data: String,
        response_tx: mpsc::Sender<Response>,
    }

    #[derive(Debug)]
    struct Response {
        id: u32,
        result: String,
    }

    let (request_tx, request_rx) = mpsc::channel::<Request>();

    // Server thread
    let server = thread::spawn(move || {
        for request in request_rx {
            println!("Server: Processing request {}: {}", request.id, request.data);
            thread::sleep(Duration::from_millis(100));

            let response = Response {
                id: request.id,
                result: format!("Processed: {}", request.data.to_uppercase()),
            };
            request.response_tx.send(response).unwrap();
        }
    });

    // Client threads
    let mut client_handles = vec![];

    for i in 0..3 {
        let request_tx = request_tx.clone();
        client_handles.push(thread::spawn(move || {
            let (response_tx, response_rx) = mpsc::channel();

            let request = Request {
                id: i,
                data: format!("hello from client {}", i),
                response_tx,
            };

            request_tx.send(request).unwrap();
            let response = response_rx.recv().unwrap();
            println!("Client {}: Got response: {:?}", i, response);
        }));
    }

    for handle in client_handles {
        handle.join().unwrap();
    }

    drop(request_tx);
    server.join().unwrap();
}

fn main() {
    basic_channel();
    multiple_producers();
    sync_channel();
    request_response();
}
