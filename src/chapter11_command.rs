//! Chapter 11: Behavioral Patterns - Command Pattern

trait Command {
    fn execute(&mut self, text: &mut String);
    fn undo(&mut self, text: &mut String);
    fn description(&self) -> String;
}

struct InsertText {
    position: usize,
    text: String,
}

impl InsertText {
    fn new(position: usize, text: &str) -> Self {
        Self {
            position,
            text: text.to_string(),
        }
    }
}

impl Command for InsertText {
    fn execute(&mut self, text: &mut String) {
        text.insert_str(self.position, &self.text);
    }

    fn undo(&mut self, text: &mut String) {
        text.drain(self.position..self.position + self.text.len());
    }

    fn description(&self) -> String {
        format!("Insert '{}' at {}", self.text, self.position)
    }
}

struct DeleteText {
    position: usize,
    length: usize,
    deleted: String,
}

impl DeleteText {
    fn new(position: usize, length: usize) -> Self {
        Self {
            position,
            length,
            deleted: String::new(),
        }
    }
}

impl Command for DeleteText {
    fn execute(&mut self, text: &mut String) {
        self.deleted = text.drain(self.position..self.position + self.length).collect();
    }

    fn undo(&mut self, text: &mut String) {
        text.insert_str(self.position, &self.deleted);
    }

    fn description(&self) -> String {
        format!("Delete {} chars at {}", self.length, self.position)
    }
}

struct TextEditor {
    content: String,
    history: Vec<Box<dyn Command>>,
    undo_stack: Vec<Box<dyn Command>>,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            content: String::new(),
            history: Vec::new(),
            undo_stack: Vec::new(),
        }
    }

    fn execute(&mut self, mut command: Box<dyn Command>) {
        println!("Execute: {}", command.description());
        command.execute(&mut self.content);
        self.history.push(command);
        self.undo_stack.clear();
    }

    fn undo(&mut self) {
        if let Some(mut command) = self.history.pop() {
            println!("Undo: {}", command.description());
            command.undo(&mut self.content);
            self.undo_stack.push(command);
        } else {
            println!("Nothing to undo");
        }
    }

    fn redo(&mut self) {
        if let Some(mut command) = self.undo_stack.pop() {
            println!("Redo: {}", command.description());
            command.execute(&mut self.content);
            self.history.push(command);
        } else {
            println!("Nothing to redo");
        }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

// Enum-based command (simpler for closed set of commands)
#[derive(Debug, Clone)]
enum EditorCommand {
    Insert { position: usize, text: String },
    Delete { position: usize, text: String },
    Replace { position: usize, old: String, new: String },
}

impl EditorCommand {
    fn apply(&self, content: &mut String) {
        match self {
            EditorCommand::Insert { position, text } => {
                content.insert_str(*position, text);
            }
            EditorCommand::Delete { position, text } => {
                content.drain(*position..*position + text.len());
            }
            EditorCommand::Replace { position, old, new } => {
                content.drain(*position..*position + old.len());
                content.insert_str(*position, new);
            }
        }
    }

    fn reverse(&self) -> EditorCommand {
        match self {
            EditorCommand::Insert { position, text } => EditorCommand::Delete {
                position: *position,
                text: text.clone(),
            },
            EditorCommand::Delete { position, text } => EditorCommand::Insert {
                position: *position,
                text: text.clone(),
            },
            EditorCommand::Replace { position, old, new } => EditorCommand::Replace {
                position: *position,
                old: new.clone(),
                new: old.clone(),
            },
        }
    }
}

fn main() {
    println!("=== Trait-Based Command Pattern ===\n");

    let mut editor = TextEditor::new();

    editor.execute(Box::new(InsertText::new(0, "Hello")));
    println!("Content: '{}'\n", editor.content());

    editor.execute(Box::new(InsertText::new(5, " World")));
    println!("Content: '{}'\n", editor.content());

    editor.execute(Box::new(DeleteText::new(5, 6)));
    println!("Content: '{}'\n", editor.content());

    editor.undo();
    println!("Content: '{}'\n", editor.content());

    editor.undo();
    println!("Content: '{}'\n", editor.content());

    editor.redo();
    println!("Content: '{}'\n", editor.content());

    println!("=== Enum-Based Command Pattern ===\n");

    let mut content = String::from("Hello World");
    println!("Initial: '{}'", content);

    let cmd = EditorCommand::Replace {
        position: 6,
        old: "World".to_string(),
        new: "Rust".to_string(),
    };

    cmd.apply(&mut content);
    println!("After replace: '{}'", content);

    let undo_cmd = cmd.reverse();
    undo_cmd.apply(&mut content);
    println!("After undo: '{}'", content);
}
