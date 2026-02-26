//! Chapter 2: Drop Trait Examples
//!
//! Demonstrates practical uses of the Drop trait for automatic cleanup,
//! timing, and scope-based actions.

use std::time::Instant;

// ============================================================================
// Timer: Automatic timing of code blocks
// ============================================================================

/// A timer that prints elapsed time when it goes out of scope.
///
/// Creating a Timer starts the clock; dropping it stops the clock
/// and prints the elapsed duration.
struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    fn new(name: &str) -> Self {
        println!("[Timer '{}'] Started", name);
        Timer {
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("[Timer '{}'] Elapsed: {:?}", self.name, elapsed);
    }
}

/// Simulates some work that takes time.
fn do_some_work(iterations: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iterations {
        sum = sum.wrapping_add(i);
    }
    sum
}

fn demo_timer() {
    println!("\n=== Timer Demo ===\n");

    // Timer starts when created, stops when scope ends
    {
        let _timer = Timer::new("inner_block");
        do_some_work(1_000_000);
        println!("Work completed inside block");
    } // Timer prints elapsed time here

    println!("After inner block\n");

    // Timer works with early returns too
    fn process_with_early_return(should_return_early: bool) -> u64 {
        let _timer = Timer::new("process_with_early_return");

        if should_return_early {
            println!("Returning early");
            return 0; // Timer still prints elapsed time
        }

        do_some_work(500_000)
    }

    let result1 = process_with_early_return(true);
    println!("Result (early): {}\n", result1);

    let result2 = process_with_early_return(false);
    println!("Result (full): {}\n", result2);
}

// ============================================================================
// ScopeGuard: Run arbitrary cleanup code when scope ends
// ============================================================================

/// A guard that runs a closure when dropped.
///
/// Useful for ensuring cleanup happens regardless of how
/// a scope is exited (normal return, early return, or panic).
struct ScopeGuard<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> ScopeGuard<F> {
    fn new(action: F) -> Self {
        ScopeGuard {
            action: Some(action),
        }
    }

    /// Disarm the guard, preventing the action from running.
    #[allow(dead_code)]
    fn disarm(&mut self) {
        self.action = None;
    }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            action();
        }
    }
}

/// Simulates a global logging state.
static mut LOGGING_ENABLED: bool = false;

fn set_logging(enabled: bool) {
    // In real code, use proper synchronization
    unsafe {
        LOGGING_ENABLED = enabled;
        println!("  [Logging {}]", if enabled { "ENABLED" } else { "DISABLED" });
    }
}

fn is_logging_enabled() -> bool {
    unsafe { LOGGING_ENABLED }
}

fn demo_scope_guard() {
    println!("\n=== ScopeGuard Demo ===\n");

    println!("Initial logging state: {}", is_logging_enabled());

    // Temporarily enable logging with guaranteed restore
    {
        set_logging(true);
        let _guard = ScopeGuard::new(|| set_logging(false));

        println!("  Inside guarded scope, logging: {}", is_logging_enabled());
        // Guard ensures logging is disabled when scope ends
    }

    println!("After guarded scope: {}\n", is_logging_enabled());

    // Guard works with early returns
    fn guarded_operation(succeed: bool) -> Result<i32, &'static str> {
        println!("  Starting guarded operation (succeed={})", succeed);
        set_logging(true);
        let _guard = ScopeGuard::new(|| {
            println!("  Guard cleanup running");
            set_logging(false);
        });

        if !succeed {
            println!("  Returning error early");
            return Err("operation failed");
        }

        println!("  Operation succeeded");
        Ok(42)
    }

    println!("Calling guarded_operation(false):");
    let _ = guarded_operation(false);
    println!("Logging after failed op: {}\n", is_logging_enabled());

    println!("Calling guarded_operation(true):");
    let _ = guarded_operation(true);
    println!("Logging after success op: {}\n", is_logging_enabled());
}

// ============================================================================
// Drop ordering demonstration
// ============================================================================

/// A value that announces when it's dropped.
struct Announcer {
    name: String,
}

impl Announcer {
    fn new(name: &str) -> Self {
        println!("  Created: {}", name);
        Announcer {
            name: name.to_string(),
        }
    }
}

impl Drop for Announcer {
    fn drop(&mut self) {
        println!("  Dropped: {}", self.name);
    }
}

fn demo_drop_ordering() {
    println!("\n=== Drop Ordering Demo ===\n");

    println!("Creating variables in order:");
    {
        let _first = Announcer::new("first");
        let _second = Announcer::new("second");
        let _third = Announcer::new("third");
        println!("\nScope ending, drops occur in reverse order:");
    }

    println!("\n--- Struct fields drop in declaration order ---\n");

    #[allow(dead_code)]
    struct Container {
        field_a: Announcer,
        field_b: Announcer,
        field_c: Announcer,
    }

    println!("Creating struct with fields a, b, c:");
    {
        let _container = Container {
            field_a: Announcer::new("field_a"),
            field_b: Announcer::new("field_b"),
            field_c: Announcer::new("field_c"),
        };
        println!("\nStruct scope ending, fields drop in declaration order:");
    }
}

// ============================================================================
// Ownership chain disposal demonstration
// ============================================================================

/// Represents a resource that logs its lifecycle.
struct Resource {
    name: String,
    children: Vec<Resource>,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("    [Resource '{}' created]", name);
        Resource {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    fn with_child(mut self, child: Resource) -> Self {
        self.children.push(child);
        self
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("    [Resource '{}' dropping, has {} children]",
                 self.name, self.children.len());
        // Children are dropped automatically after this
    }
}

fn demo_ownership_chain_disposal() {
    println!("\n=== Ownership Chain Disposal Demo ===\n");

    println!("Building ownership tree:");
    {
        let root = Resource::new("root")
            .with_child(
                Resource::new("child_1")
                    .with_child(Resource::new("grandchild_1a"))
                    .with_child(Resource::new("grandchild_1b"))
            )
            .with_child(
                Resource::new("child_2")
                    .with_child(Resource::new("grandchild_2a"))
            );

        println!("\nTree built. Ownership chain:");
        println!("  root");
        println!("  ├── child_1");
        println!("  │   ├── grandchild_1a");
        println!("  │   └── grandchild_1b");
        println!("  └── child_2");
        println!("      └── grandchild_2a");
        println!("\nDropping root (entire tree will be disposed):");
        drop(root);
    }

    println!("\nAll resources freed automatically through ownership chain.");
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("Chapter 2: Drop Trait Examples");
    println!("==============================");

    demo_timer();
    demo_scope_guard();
    demo_drop_ordering();
    demo_ownership_chain_disposal();

    println!("\n=== All demos complete ===");
}
