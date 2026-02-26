//! Chapter 11: Behavioral Patterns - State Pattern (Typestate)

use std::marker::PhantomData;

// Typestate pattern for document workflow
mod typestate {
    use super::*;

    pub struct Draft;
    pub struct PendingReview;
    pub struct Approved;
    pub struct Rejected;

    pub struct Document<State> {
        content: String,
        _state: PhantomData<State>,
    }

    impl Document<Draft> {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.to_string(),
                _state: PhantomData,
            }
        }

        pub fn edit(&mut self, new_content: &str) {
            self.content = new_content.to_string();
        }

        pub fn submit_for_review(self) -> Document<PendingReview> {
            println!("Document submitted for review");
            Document {
                content: self.content,
                _state: PhantomData,
            }
        }
    }

    impl Document<PendingReview> {
        pub fn approve(self) -> Document<Approved> {
            println!("Document approved");
            Document {
                content: self.content,
                _state: PhantomData,
            }
        }

        pub fn reject(self, reason: &str) -> Document<Rejected> {
            println!("Document rejected: {}", reason);
            Document {
                content: self.content,
                _state: PhantomData,
            }
        }
    }

    impl Document<Approved> {
        pub fn publish(&self) {
            println!("Publishing: {}", self.content);
        }
    }

    impl Document<Rejected> {
        pub fn revise(self) -> Document<Draft> {
            println!("Document returned to draft for revision");
            Document {
                content: self.content,
                _state: PhantomData,
            }
        }
    }

    impl<State> Document<State> {
        pub fn content(&self) -> &str {
            &self.content
        }
    }
}

// Enum-based state machine
mod enum_state {
    #[derive(Debug, Clone)]
    pub enum ConnectionState {
        Disconnected,
        Connecting { attempt: u32 },
        Connected { session_id: String },
        Failed { error: String },
    }

    pub struct Connection {
        state: ConnectionState,
    }

    impl Connection {
        pub fn new() -> Self {
            Self {
                state: ConnectionState::Disconnected,
            }
        }

        pub fn connect(&mut self) {
            let new_state = match &self.state {
                ConnectionState::Disconnected | ConnectionState::Failed { .. } => {
                    println!("Connecting (attempt 1)...");
                    Some(ConnectionState::Connecting { attempt: 1 })
                }
                ConnectionState::Connecting { attempt } => {
                    let next = attempt + 1;
                    println!("Retrying (attempt {})...", next);
                    Some(ConnectionState::Connecting { attempt: next })
                }
                ConnectionState::Connected { .. } => {
                    println!("Already connected");
                    None
                }
            };
            if let Some(state) = new_state {
                self.state = state;
            }
        }

        pub fn on_success(&mut self, session_id: &str) {
            if let ConnectionState::Connecting { .. } = &self.state {
                self.state = ConnectionState::Connected {
                    session_id: session_id.to_string(),
                };
                println!("Connected with session: {}", session_id);
            }
        }

        pub fn on_failure(&mut self, error: &str) {
            if let ConnectionState::Connecting { .. } = &self.state {
                self.state = ConnectionState::Failed {
                    error: error.to_string(),
                };
                println!("Connection failed: {}", error);
            }
        }

        pub fn disconnect(&mut self) {
            match &self.state {
                ConnectionState::Connected { session_id } => {
                    println!("Disconnecting session: {}", session_id);
                    self.state = ConnectionState::Disconnected;
                }
                _ => {
                    self.state = ConnectionState::Disconnected;
                }
            }
        }

        pub fn state(&self) -> &ConnectionState {
            &self.state
        }
    }

    impl Default for Connection {
        fn default() -> Self {
            Self::new()
        }
    }
}

fn main() {
    println!("=== Typestate Document Workflow ===\n");

    let mut doc = typestate::Document::<typestate::Draft>::new("Initial content");
    println!("Draft content: {}", doc.content());

    doc.edit("Updated content");
    println!("After edit: {}", doc.content());

    let pending = doc.submit_for_review();
    // doc.edit("Can't edit"); // Won't compile - not in Draft state

    let approved = pending.approve();
    // pending.reject("reason"); // Won't compile - pending was moved

    approved.publish();

    println!("\n=== Rejection Flow ===\n");

    let doc2 = typestate::Document::<typestate::Draft>::new("Bad content");
    let pending2 = doc2.submit_for_review();
    let rejected = pending2.reject("Needs more detail");
    let mut revised = rejected.revise();
    revised.edit("Better content");
    let pending3 = revised.submit_for_review();
    let approved2 = pending3.approve();
    approved2.publish();

    println!("\n=== Enum-Based Connection State ===\n");

    let mut conn = enum_state::Connection::new();
    println!("Initial state: {:?}", conn.state());

    conn.connect();
    println!("State: {:?}", conn.state());

    conn.on_success("sess_abc123");
    println!("State: {:?}", conn.state());

    conn.disconnect();
    println!("State: {:?}", conn.state());

    conn.connect();
    conn.on_failure("Network timeout");
    println!("State: {:?}", conn.state());
}
