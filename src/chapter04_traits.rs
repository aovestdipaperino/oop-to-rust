//! Chapter 4: Traits - Contracts Without Inheritance
//!
//! This example demonstrates traits, default implementations,
//! trait bounds, and static vs dynamic dispatch.

use std::fmt::Display;

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}: {}", self.username, self.content)
    }
}

// Static dispatch
fn notify<T: Summary>(item: &T) {
    println!("[Static] Breaking news: {}", item.summarize());
}

// Multiple trait bounds
fn notify_and_display<T: Summary + Display>(item: &T) {
    println!("[Static+Display] {}: {}", item, item.summarize());
}

// Dynamic dispatch
fn notify_dynamic(item: &dyn Summary) {
    println!("[Dynamic] Breaking news: {}", item.summarize());
}

fn create_feed() -> Vec<Box<dyn Summary>> {
    vec![
        Box::new(NewsArticle {
            headline: "Rust 2024 Edition Released".to_string(),
            location: "San Francisco".to_string(),
            author: "Jane Developer".to_string(),
            content: "The Rust team has announced...".to_string(),
        }),
        Box::new(Tweet {
            username: "rustlang".to_string(),
            content: "Rust 2024 is here!".to_string(),
        }),
    ]
}

// Supertraits
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Tweet {}

// Associated types
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

struct VecContainer<T> {
    items: Vec<T>,
}

impl<T> VecContainer<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T> Container for VecContainer<T> {
    type Item = T;

    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.items.get(index)
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    println!("=== Traits and Default Implementations ===\n");

    let article = NewsArticle {
        headline: "Scientists Discover New Species".to_string(),
        location: "Amazon Rainforest".to_string(),
        author: "Dr. Maria Santos".to_string(),
        content: "In a groundbreaking discovery...".to_string(),
    };

    let tweet = Tweet {
        username: "nature_news".to_string(),
        content: "Amazing discovery in the Amazon!".to_string(),
    };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());

    println!("\n=== Static vs Dynamic Dispatch ===\n");

    notify(&article);
    notify(&tweet);
    notify_dynamic(&article);
    notify_dynamic(&tweet);

    println!("\n=== Multiple Trait Bounds ===\n");
    notify_and_display(&tweet);

    println!("\n=== Heterogeneous Collections ===\n");
    let feed = create_feed();
    for (i, item) in feed.iter().enumerate() {
        println!("  {}: {}", i + 1, item.summarize());
    }

    println!("\n=== Supertraits ===\n");
    tweet.outline_print();

    println!("\n=== Associated Types ===\n");
    let mut container: VecContainer<String> = VecContainer::new();
    container.add("first".to_string());
    container.add("second".to_string());
    println!("Container has {} items", container.len());
    if let Some(item) = container.get(1) {
        println!("Item at index 1: {}", item);
    }
}
