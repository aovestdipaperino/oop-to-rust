# From OOP to Rust -- Companion Code

This repository contains all runnable code examples from the book [**From OOP to Rust**](https://github.com/aovestdipaperino/oop-to-rust), a guide for experienced object-oriented developers making the transition to Rust.

Each example is a self-contained binary you can run independently. No need to hunt through a monorepo or stitch files together; pick a chapter, run it, and read the output alongside the book.

## Prerequisites

- **Rust 1.85+** (edition 2024) -- install via [rustup](https://rustup.rs/)

## Quick Start

Clone the repository and run any example by chapter name:

```bash
git clone https://github.com/aovestdipaperino/oop-to-rust.git
cd oop-to-rust
cargo run --bin chapter04-traits
```

To see all available examples:

```bash
cargo run --bin
```

## Examples by Chapter

| Binary | Chapter | Topic |
|--------|---------|-------|
| `chapter02-drop` | Rust Fundamentals | Ownership, Drop, and RAII |
| `chapter02-lifetimes` | Rust Fundamentals | Lifetimes and borrowing |
| `chapter03-enums` | Structs, Enums, and the Death of Null | Algebraic data types |
| `chapter03-options` | Structs, Enums, and the Death of Null | Option as null replacement |
| `chapter04-traits` | Traits | Contracts without inheritance |
| `chapter06-errors` | Error Handling | Result, thiserror, anyhow |
| `chapter09-builder` | Creational Patterns | Type-state builder pattern |
| `chapter09-factory` | Creational Patterns | Factory functions and traits |
| `chapter10-composite` | Structural Patterns | Composite with enum dispatch |
| `chapter10-decorator` | Structural Patterns | Decorator via trait wrapping |
| `chapter11-command` | Behavioral Patterns | Command pattern |
| `chapter11-state` | Behavioral Patterns | Type-state machines |
| `chapter11-strategy` | Behavioral Patterns | Strategy via closures and traits |
| `chapter13-shared-state` | Concurrency Foundations | Arc, Mutex, and shared state |
| `chapter13-cache` | Concurrency Foundations | Concurrent caching with DashMap |
| `chapter14-channels` | Message Passing | Channel-based communication |
| `chapter14-pipeline` | Message Passing | Multi-stage pipelines |
| `chapter14-actor` | Message Passing | Actor model |
| `chapter15-async-basics` | Async Rust | Tokio, futures, and async/await |
| `chapter16-cancellation` | Cancellation | Graceful shutdown patterns |
| `chapter17-atomics` | Concurrent Data Structures | Atomic operations and ordering |
| `chapter17-work-stealing` | Concurrent Data Structures | Work-stealing scheduler |

## Dependencies

The examples use a curated set of crates that reflect real-world Rust practice:

- **tokio** / **tokio-util** for async runtime
- **anyhow** / **thiserror** for error handling
- **crossbeam** / **dashmap** for concurrency primitives
- **serde** / **serde_json** for serialization
- **reqwest** for HTTP
- **proptest** for property-based testing

## About the Book

*From OOP to Rust* is written for developers who have spent years (maybe a decade or more) building systems in Java, C#, or C++. It is not a gentle introduction to programming. It is a translation layer: a way to map the mental models you already have onto Rust's ownership system, trait-based polymorphism, and fearless concurrency, and guidance on when those models must be abandoned entirely.

The book covers ownership and borrowing, error handling without exceptions, design patterns reimagined for Rust, and a full section on concurrent and asynchronous programming.

## Thank You

This repository exists because people read the book, and that still feels a bit unreal to me.

Writing *From OOP to Rust* started as a way to organize my own notes after years of stumbling through the transition from Java and C#. It turned into something much larger, and much more personal, than I expected. Every chapter carries the memory of a real mistake I made, a concept that took me far too long to grasp, or a moment where Rust's compiler finally stopped fighting me and started feeling like a collaborator.

If you are here, it means you are investing your time in learning something new. That is not a small thing. Careers get comfortable, habits calcify, and picking up a language that will reject your first fifty attempts at writing a linked list takes genuine courage (or stubbornness, which is the same thing).

I hope these examples save you some of the late nights I spent staring at borrow checker errors. And if they do not, I hope they at least give you the confidence that those errors are solvable, that the patterns do click eventually, and that the effort is worth it.

Thank you for reading. Thank you for running `cargo build` on code a stranger wrote. If any of this helps you build something meaningful, then every hour I spent on it was time well spent.

May your builds be clean and your borrow checker errors instructive.

-- Enzo

## License

MIT
