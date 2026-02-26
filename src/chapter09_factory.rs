//! Chapter 9: Creational Patterns - Factory Pattern

trait Document: std::fmt::Debug {
    fn render(&self) -> String;
    fn doc_type(&self) -> &str;
}

#[derive(Debug)]
struct PdfDocument {
    content: String,
}

impl Document for PdfDocument {
    fn render(&self) -> String {
        format!("[PDF] {}", self.content)
    }
    fn doc_type(&self) -> &str {
        "PDF"
    }
}

#[derive(Debug)]
struct HtmlDocument {
    content: String,
}

impl Document for HtmlDocument {
    fn render(&self) -> String {
        format!("<html><body>{}</body></html>", self.content)
    }
    fn doc_type(&self) -> &str {
        "HTML"
    }
}

fn create_document(doc_type: &str, content: &str) -> Option<Box<dyn Document>> {
    match doc_type.to_lowercase().as_str() {
        "pdf" => Some(Box::new(PdfDocument {
            content: content.to_string(),
        })),
        "html" => Some(Box::new(HtmlDocument {
            content: content.to_string(),
        })),
        _ => None,
    }
}

// Abstract Factory
trait Button: std::fmt::Debug {
    fn click(&self);
    fn render(&self) -> String;
}

trait UiFactory {
    fn create_button(&self, label: &str) -> Box<dyn Button>;
}

#[derive(Debug)]
struct WindowsButton {
    label: String,
}

impl Button for WindowsButton {
    fn click(&self) {
        println!("[Windows] Button '{}' clicked!", self.label);
    }
    fn render(&self) -> String {
        format!("[Windows Button: {}]", self.label)
    }
}

struct WindowsFactory;

impl UiFactory for WindowsFactory {
    fn create_button(&self, label: &str) -> Box<dyn Button> {
        Box::new(WindowsButton {
            label: label.to_string(),
        })
    }
}

#[derive(Debug)]
struct MacButton {
    label: String,
}

impl Button for MacButton {
    fn click(&self) {
        println!("[macOS] Button '{}' clicked!", self.label);
    }
    fn render(&self) -> String {
        format!("(macOS Button: {})", self.label)
    }
}

struct MacFactory;

impl UiFactory for MacFactory {
    fn create_button(&self, label: &str) -> Box<dyn Button> {
        Box::new(MacButton {
            label: label.to_string(),
        })
    }
}

fn get_ui_factory(platform: &str) -> Box<dyn UiFactory> {
    match platform.to_lowercase().as_str() {
        "windows" => Box::new(WindowsFactory),
        _ => Box::new(MacFactory),
    }
}

// Enum-based factory
#[derive(Debug, Clone)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn circle(radius: f64) -> Self {
        Shape::Circle { radius }
    }

    fn rectangle(width: f64, height: f64) -> Self {
        Shape::Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

fn main() {
    println!("=== Simple Factory ===\n");

    for doc_type in ["pdf", "html", "unknown"] {
        match create_document(doc_type, "Hello, World!") {
            Some(doc) => println!("{}: {}", doc.doc_type(), doc.render()),
            None => println!("Unknown document type: {}", doc_type),
        }
    }

    println!("\n=== Abstract Factory ===\n");

    for platform in ["windows", "macos"] {
        let factory = get_ui_factory(platform);
        let button = factory.create_button("Submit");
        println!("{}", button.render());
        button.click();
    }

    println!("\n=== Enum-Based Factory ===\n");

    let shapes = vec![Shape::circle(5.0), Shape::rectangle(4.0, 6.0)];

    for shape in &shapes {
        println!("{:?} - Area: {:.2}", shape, shape.area());
    }
}
