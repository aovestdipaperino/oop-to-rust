//! Chapter 14: Message Passing - Pipeline Pattern

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

fn spawn_stage<T, U, F>(
    name: &'static str,
    input: Receiver<T>,
    transform: F,
) -> (Receiver<U>, JoinHandle<()>)
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> Option<U> + Send + 'static,
{
    let (output_tx, output_rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for item in input {
            if let Some(result) = transform(item) {
                if output_tx.send(result).is_err() {
                    break;
                }
            }
        }
        println!("[{}] Stage finished", name);
    });

    (output_rx, handle)
}

#[derive(Debug, Clone)]
struct LogEntry {
    level: String,
    message: String,
    timestamp: u64,
}

fn main() {
    println!("=== Data Processing Pipeline ===\n");

    // Create input channel
    let (input_tx, input_rx) = mpsc::channel::<String>();

    // Stage 1: Parse log entries
    let (parsed_rx, parse_handle) = spawn_stage("parser", input_rx, |line: String| {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() == 3 {
            Some(LogEntry {
                timestamp: parts[0].parse().unwrap_or(0),
                level: parts[1].to_string(),
                message: parts[2].to_string(),
            })
        } else {
            println!("[parser] Skipping invalid line: {}", line);
            None
        }
    });

    // Stage 2: Filter (only ERROR and WARN)
    let (filtered_rx, filter_handle) = spawn_stage("filter", parsed_rx, |entry: LogEntry| {
        if entry.level == "ERROR" || entry.level == "WARN" {
            Some(entry)
        } else {
            None
        }
    });

    // Stage 3: Transform (format output)
    let (formatted_rx, format_handle) = spawn_stage("formatter", filtered_rx, |entry: LogEntry| {
        Some(format!(
            "[{}] {} - {}",
            entry.timestamp, entry.level, entry.message
        ))
    });

    // Collector
    let collector = thread::spawn(move || {
        let mut count = 0;
        for formatted in formatted_rx {
            println!("Output: {}", formatted);
            count += 1;
        }
        println!("\n[collector] Processed {} entries", count);
    });

    // Feed input
    let log_lines = vec![
        "1001|INFO|Application started",
        "1002|DEBUG|Loading configuration",
        "1003|WARN|Config file not found, using defaults",
        "1004|ERROR|Database connection failed",
        "1005|INFO|Retrying connection",
        "1006|ERROR|Retry failed",
        "1007|INFO|Shutting down",
        "invalid line without separators",
        "1008|WARN|Cleanup incomplete",
    ];

    println!("Feeding {} log lines into pipeline...\n", log_lines.len());

    for line in log_lines {
        input_tx.send(line.to_string()).unwrap();
    }

    drop(input_tx); // Signal end of input

    // Wait for pipeline to complete
    parse_handle.join().unwrap();
    filter_handle.join().unwrap();
    format_handle.join().unwrap();
    collector.join().unwrap();

    println!("\nPipeline completed!");
}
