//! Chapter 2: Lifetime Examples
//!
//! Demonstrates Rust's lifetime system for ensuring reference validity.

use std::borrow::Cow;

// ============================================================================
// Functions Returning References
// ============================================================================

/// Returns a reference to the longer of two string slices.
///
/// The lifetime parameter 'a ties the output lifetime to both inputs,
/// ensuring the returned reference remains valid as long as both inputs do.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn demo_longest() {
    println!("=== Functions Returning References ===\n");

    let s1 = String::from("long string");
    let s2 = String::from("short");

    let result = longest(&s1, &s2);
    println!("longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // The result is valid because both s1 and s2 are still in scope
    println!("Result: {}\n", result);

    // Demonstrating lifetime constraints
    let outer = String::from("outer string");
    {
        let inner = String::from("inner");
        let result = longest(&outer, &inner);
        println!("Inside inner scope: {}", result);
        // result is valid here because both outer and inner exist
    }
    // inner is dropped here, so we cannot use a result that might reference it
    println!("After inner scope: outer still valid = {}\n", outer);
}

// ============================================================================
// Divergent Lifetimes with Cow
// ============================================================================

/// Returns the longer string, cloning to avoid lifetime issues.
///
/// When inputs have independent lifetimes, returning owned data
/// sidesteps the lifetime constraints entirely.
fn pick_one_owned<'a, 'b>(x: &'a str, y: &'b str) -> Cow<'static, str> {
    if x.len() > y.len() {
        Cow::Owned(x.to_owned())
    } else {
        Cow::Owned(y.to_owned())
    }
}

/// Returns the longer string, borrowing when lifetimes align.
///
/// When both inputs share the same lifetime, we can borrow without cloning.
fn pick_one_borrowed<'a>(x: &'a str, y: &'a str) -> Cow<'a, str> {
    if x.len() > y.len() {
        Cow::Borrowed(x)
    } else {
        Cow::Borrowed(y)
    }
}

fn demo_cow() {
    println!("=== Divergent Lifetimes with Cow ===\n");

    let s1 = String::from("hello world");
    let s2 = String::from("hi");

    // Using owned version (always clones)
    let owned_result = pick_one_owned(&s1, &s2);
    println!("pick_one_owned: {} (is_owned: {})",
             owned_result, matches!(owned_result, Cow::Owned(_)));

    // Using borrowed version (no clone when lifetimes match)
    let borrowed_result = pick_one_borrowed(&s1, &s2);
    println!("pick_one_borrowed: {} (is_borrowed: {})",
             borrowed_result, matches!(borrowed_result, Cow::Borrowed(_)));

    // Cow can be converted to owned String when needed
    let owned_string: String = borrowed_result.into_owned();
    println!("Converted to owned: {}\n", owned_string);
}

// ============================================================================
// Structs Holding References
// ============================================================================

/// A struct that holds a reference to string data.
///
/// The lifetime parameter ensures Holder cannot outlive its data.
#[derive(Debug)]
struct Holder<'a> {
    data: &'a str,
}

impl<'a> Holder<'a> {
    fn new(data: &'a str) -> Self {
        Holder { data }
    }

    fn get(&self) -> &str {
        self.data
    }
}

/// Demonstrates a struct with multiple reference fields.
#[derive(Debug)]
#[allow(dead_code)]
struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

fn demo_structs_with_references() {
    println!("=== Structs Holding References ===\n");

    let text = String::from("Hello, lifetime world");

    // Holder borrows from text
    let holder = Holder::new(&text);
    println!("Holder contains: {:?}", holder);
    println!("Holder.get(): {}", holder.get());

    // Holder must not outlive text
    // If we dropped text here, holder would be invalid
    // drop(text); // This would cause a compile error

    // Multiple lifetimes in a struct
    let s1 = String::from("first");
    let s2 = String::from("second");

    let pair = Pair {
        first: &s1,
        second: &s2,
    };
    println!("Pair: {:?}\n", pair);
}

// ============================================================================
// Lifetime Elision
// ============================================================================

/// First word extraction - lifetime elided by compiler.
///
/// The compiler infers: fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// A document with a title.
struct Document {
    title: String,
    content: String,
}

impl Document {
    fn new(title: &str, content: &str) -> Self {
        Document {
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    /// Returns the title - lifetime elided, inferred to borrow from self.
    fn title(&self) -> &str {
        &self.title
    }

    /// Returns content length - no lifetime needed (returns owned value).
    fn content_length(&self) -> usize {
        self.content.len()
    }

    /// Returns a summary - lifetime explicitly tied to self.
    fn summary(&self) -> &str {
        if self.content.len() > 50 {
            &self.content[..50]
        } else {
            &self.content
        }
    }
}

fn demo_elision() {
    println!("=== Lifetime Elision ===\n");

    let sentence = "Hello world from Rust";
    println!("Sentence: \"{}\"", sentence);
    println!("First word: \"{}\"\n", first_word(sentence));

    let doc = Document::new(
        "Rust Lifetimes",
        "Lifetimes ensure references remain valid."
    );

    println!("Document title: {}", doc.title());
    println!("Content length: {}", doc.content_length());
    println!("Summary: {}\n", doc.summary());
}

// ============================================================================
// The 'static Lifetime
// ============================================================================

/// Returns a static string literal.
fn static_string() -> &'static str {
    "I am embedded in the binary"
}

/// Demonstrates thread spawning with 'static requirement.
fn demo_static_lifetime() {
    println!("=== The 'static Lifetime ===\n");

    // String literals are 'static
    let literal: &'static str = "I live for the entire program";
    println!("Static literal: {}", literal);

    // Function returning 'static
    println!("From function: {}\n", static_string());

    // Thread spawning requires 'static
    let owned = String::from("owned data for thread");

    let handle = std::thread::spawn(move || {
        // owned is moved into the closure, satisfying 'static
        println!("Thread received: {}", owned);
        owned.len()
    });

    let result = handle.join().unwrap();
    println!("Thread returned: {}\n", result);
}

// ============================================================================
// Common Mistakes (Demonstrating What NOT to Do)
// ============================================================================

// These functions show patterns that would NOT compile if uncommented:

/*
// ERROR: Returns reference to local variable
fn create_greeting() -> &str {
    let s = String::from("hello");
    &s // s is dropped when function returns
}

// ERROR: Struct outlives its data
fn bad_holder() -> Holder<'static> {
    let s = String::from("temporary");
    Holder { data: &s } // s does not live long enough
}
*/

/// Correct version: return owned data instead of reference
fn create_greeting_correct() -> String {
    String::from("hello")
}

/// Correct version: ensure data outlives the holder
fn good_holder(data: &str) -> Holder<'_> {
    Holder::new(data)
}

fn demo_correct_patterns() {
    println!("=== Correct Patterns (Avoiding Common Mistakes) ===\n");

    // Return owned data instead of references to locals
    let greeting = create_greeting_correct();
    println!("Greeting (owned): {}", greeting);

    // Ensure data outlives the struct
    let data = String::from("long-lived data");
    let holder = good_holder(&data);
    println!("Holder with valid lifetime: {:?}", holder);

    // data is still valid here
    println!("Original data still accessible: {}\n", data);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("Chapter 2: Lifetime Examples");
    println!("============================\n");

    demo_longest();
    demo_cow();
    demo_structs_with_references();
    demo_elision();
    demo_static_lifetime();
    demo_correct_patterns();

    println!("=== All lifetime demos complete ===");
}
