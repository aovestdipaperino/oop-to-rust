//! Chapter 10: Structural Patterns - Decorator Pattern

use std::time::{Duration, Instant};

trait Notifier: Send + Sync {
    fn send(&self, message: &str) -> Result<(), String>;
    fn name(&self) -> &str;
}

struct EmailNotifier {
    email: String,
}

impl EmailNotifier {
    fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("  [Email] Sending to {}: {}", self.email, message);
        Ok(())
    }
    fn name(&self) -> &str {
        "EmailNotifier"
    }
}

struct LoggingNotifier<N: Notifier> {
    inner: N,
}

impl<N: Notifier> LoggingNotifier<N> {
    fn new(notifier: N) -> Self {
        Self { inner: notifier }
    }
}

impl<N: Notifier + Send + Sync> Notifier for LoggingNotifier<N> {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("  [Log] {} sending: {}", self.inner.name(), message);
        let result = self.inner.send(message);
        match &result {
            Ok(()) => println!("  [Log] Success"),
            Err(e) => println!("  [Log] Failed: {}", e),
        }
        result
    }
    fn name(&self) -> &str {
        "LoggingNotifier"
    }
}

struct RetryNotifier<N: Notifier> {
    inner: N,
    max_attempts: u32,
    delay: Duration,
}

impl<N: Notifier> RetryNotifier<N> {
    fn new(notifier: N, max_attempts: u32, delay: Duration) -> Self {
        Self {
            inner: notifier,
            max_attempts,
            delay,
        }
    }
}

impl<N: Notifier + Send + Sync> Notifier for RetryNotifier<N> {
    fn send(&self, message: &str) -> Result<(), String> {
        for attempt in 1..=self.max_attempts {
            println!("  [Retry] Attempt {}/{}", attempt, self.max_attempts);
            match self.inner.send(message) {
                Ok(()) => return Ok(()),
                Err(e) if attempt < self.max_attempts => {
                    println!("  [Retry] Failed ({}), waiting...", e);
                    std::thread::sleep(self.delay);
                }
                Err(e) => return Err(format!("All attempts failed: {}", e)),
            }
        }
        unreachable!()
    }
    fn name(&self) -> &str {
        "RetryNotifier"
    }
}

struct TimingNotifier<N: Notifier> {
    inner: N,
}

impl<N: Notifier> TimingNotifier<N> {
    fn new(notifier: N) -> Self {
        Self { inner: notifier }
    }
}

impl<N: Notifier + Send + Sync> Notifier for TimingNotifier<N> {
    fn send(&self, message: &str) -> Result<(), String> {
        let start = Instant::now();
        let result = self.inner.send(message);
        println!("  [Timing] Operation took {:?}", start.elapsed());
        result
    }
    fn name(&self) -> &str {
        "TimingNotifier"
    }
}

fn main() {
    println!("=== Basic Notifier ===\n");
    let email = EmailNotifier::new("user@example.com");
    email.send("Hello!").unwrap();

    println!("\n=== Logging Decorator ===\n");
    let logged = LoggingNotifier::new(EmailNotifier::new("user@example.com"));
    logged.send("Hello with logging!").unwrap();

    println!("\n=== Timing Decorator ===\n");
    let timed = TimingNotifier::new(EmailNotifier::new("user@example.com"));
    timed.send("Hello with timing!").unwrap();

    println!("\n=== Composed Decorators ===\n");
    let composed = TimingNotifier::new(LoggingNotifier::new(EmailNotifier::new("user@example.com")));
    composed.send("Hello with both!").unwrap();

    println!("\n=== Full Stack ===\n");
    let full = TimingNotifier::new(LoggingNotifier::new(RetryNotifier::new(
        EmailNotifier::new("ceo@company.com"),
        2,
        Duration::from_millis(50),
    )));
    full.send("Critical notification!").unwrap();
}
