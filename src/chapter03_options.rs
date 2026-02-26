//! Chapter 3: Option and Result
//!
//! This example demonstrates working with Option and Result types,
//! showing various patterns for handling absence and errors.

use std::collections::HashMap;

// A simple user database
struct UserDatabase {
    users: HashMap<u64, User>,
}

#[derive(Debug, Clone)]
struct User {
    id: u64,
    username: String,
    email: Option<String>, // Email is optional
}

#[derive(Debug)]
enum DatabaseError {
    NotFound,
    DuplicateId,
    InvalidData(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::NotFound => write!(f, "User not found"),
            DatabaseError::DuplicateId => write!(f, "User ID already exists"),
            DatabaseError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl UserDatabase {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, user: User) -> Result<(), DatabaseError> {
        if user.username.is_empty() {
            return Err(DatabaseError::InvalidData(
                "Username cannot be empty".to_string(),
            ));
        }

        if self.users.contains_key(&user.id) {
            return Err(DatabaseError::DuplicateId);
        }

        self.users.insert(user.id, user);
        Ok(())
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    fn get_user_or_error(&self, id: u64) -> Result<&User, DatabaseError> {
        self.users.get(&id).ok_or(DatabaseError::NotFound)
    }

    fn find_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    fn get_user_email(&self, id: u64) -> Option<&str> {
        self.get_user(id)?.email.as_deref()
    }
}

fn demonstrate_option_combinators() {
    println!("=== Option Combinators ===\n");

    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    let doubled = some_number.map(|n| n * 2);
    println!("Some(42).map(|n| n * 2) = {:?}", doubled);

    let doubled_none = no_number.map(|n| n * 2);
    println!("None.map(|n| n * 2) = {:?}", doubled_none);

    let value = some_number.unwrap_or(0);
    println!("Some(42).unwrap_or(0) = {}", value);

    let value = no_number.unwrap_or(0);
    println!("None.unwrap_or(0) = {}", value);

    let value = no_number.unwrap_or_else(|| {
        println!("  Computing default...");
        100
    });
    println!("None.unwrap_or_else(|| 100) = {}", value);

    let result = some_number
        .and_then(|n| if n > 0 { Some(n * 2) } else { None })
        .and_then(|n| if n < 100 { Some(n + 1) } else { None });
    println!("Chained and_then: {:?}", result);

    let filtered = some_number.filter(|&n| n > 50);
    println!("Some(42).filter(|n| n > 50) = {:?}", filtered);

    let filtered = some_number.filter(|&n| n > 10);
    println!("Some(42).filter(|n| n > 10) = {:?}", filtered);
}

fn demonstrate_result_combinators() {
    println!("\n=== Result Combinators ===\n");

    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("something went wrong");

    let doubled = ok_result.map(|n| n * 2);
    println!("Ok(42).map(|n| n * 2) = {:?}", doubled);

    let mapped_err = err_result.map_err(|e| format!("Error: {}", e));
    println!("Err(...).map_err(...) = {:?}", mapped_err);

    let value = err_result.unwrap_or(0);
    println!("Err(...).unwrap_or(0) = {}", value);

    let as_option = ok_result.ok();
    println!("Ok(42).ok() = {:?}", as_option);

    let as_option = err_result.ok();
    println!("Err(...).ok() = {:?}", as_option);
}

fn fetch_user_email(db: &UserDatabase, user_id: u64) -> Result<String, DatabaseError> {
    let user = db.get_user_or_error(user_id)?;
    let email = user
        .email
        .clone()
        .ok_or(DatabaseError::InvalidData("No email set".to_string()))?;
    Ok(email)
}

fn main() {
    demonstrate_option_combinators();
    demonstrate_result_combinators();

    println!("\n=== Database Operations ===\n");

    let mut db = UserDatabase::new();

    let users = vec![
        User {
            id: 1,
            username: "alice".to_string(),
            email: Some("alice@example.com".to_string()),
        },
        User {
            id: 2,
            username: "bob".to_string(),
            email: None,
        },
        User {
            id: 3,
            username: "charlie".to_string(),
            email: Some("charlie@example.com".to_string()),
        },
    ];

    for user in users {
        match db.add_user(user.clone()) {
            Ok(()) => println!("Added user: {}", user.username),
            Err(e) => println!("Failed to add user: {}", e),
        }
    }

    println!("\nAttempting to add duplicate ID:");
    let duplicate = User {
        id: 1,
        username: "duplicate".to_string(),
        email: None,
    };
    match db.add_user(duplicate) {
        Ok(()) => println!("Added user"),
        Err(e) => println!("Failed: {}", e),
    }

    println!("\n--- Querying users ---");

    match db.get_user(1) {
        Some(user) => println!("Found user 1: {:?}", user),
        None => println!("User 1 not found"),
    }

    match db.get_user(999) {
        Some(user) => println!("Found user 999: {:?}", user),
        None => println!("User 999 not found"),
    }

    if let Some(user) = db.find_by_username("bob") {
        println!("Found bob: {:?}", user);
    }

    println!("\n--- Getting emails ---");
    println!("User 1 email: {:?}", db.get_user_email(1));
    println!("User 2 email: {:?}", db.get_user_email(2));
    println!("User 999 email: {:?}", db.get_user_email(999));

    println!("\n--- Using ? operator ---");
    match fetch_user_email(&db, 1) {
        Ok(email) => println!("User 1 email via ?: {}", email),
        Err(e) => println!("Error: {}", e),
    }

    match fetch_user_email(&db, 2) {
        Ok(email) => println!("User 2 email via ?: {}", email),
        Err(e) => println!("Error: {}", e),
    }

    match fetch_user_email(&db, 999) {
        Ok(email) => println!("User 999 email via ?: {}", email),
        Err(e) => println!("Error: {}", e),
    }
}
