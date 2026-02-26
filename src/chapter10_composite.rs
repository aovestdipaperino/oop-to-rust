//! Chapter 10: Structural Patterns - Composite Pattern

use std::fmt;

#[derive(Clone)]
enum FileEntry {
    File { name: String, size: u64 },
    Directory { name: String, children: Vec<FileEntry> },
}

impl FileEntry {
    fn file(name: &str, size: u64) -> Self {
        FileEntry::File {
            name: name.to_string(),
            size,
        }
    }

    fn directory(name: &str, children: Vec<FileEntry>) -> Self {
        FileEntry::Directory {
            name: name.to_string(),
            children,
        }
    }

    fn name(&self) -> &str {
        match self {
            FileEntry::File { name, .. } => name,
            FileEntry::Directory { name, .. } => name,
        }
    }

    fn size(&self) -> u64 {
        match self {
            FileEntry::File { size, .. } => *size,
            FileEntry::Directory { children, .. } => children.iter().map(|c| c.size()).sum(),
        }
    }

    fn count_files(&self) -> usize {
        match self {
            FileEntry::File { .. } => 1,
            FileEntry::Directory { children, .. } => {
                children.iter().map(|c| c.count_files()).sum()
            }
        }
    }

    fn print_tree(&self, prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        let icon = match self {
            FileEntry::File { .. } => "📄",
            FileEntry::Directory { .. } => "📁",
        };

        println!("{}{}{} {}", prefix, connector, icon, self.name());

        if let FileEntry::Directory { children, .. } = self {
            let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
            for (i, child) in children.iter().enumerate() {
                child.print_tree(&new_prefix, i == children.len() - 1);
            }
        }
    }
}

impl fmt::Debug for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileEntry::File { name, size } => write!(f, "File({}, {} bytes)", name, size),
            FileEntry::Directory { name, children } => {
                write!(f, "Dir({}, {} items)", name, children.len())
            }
        }
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn main() {
    let project = FileEntry::directory(
        "my-project",
        vec![
            FileEntry::file("Cargo.toml", 512),
            FileEntry::file("README.md", 2048),
            FileEntry::directory(
                "src",
                vec![
                    FileEntry::file("main.rs", 1024),
                    FileEntry::file("lib.rs", 4096),
                    FileEntry::directory(
                        "models",
                        vec![
                            FileEntry::file("mod.rs", 256),
                            FileEntry::file("user.rs", 2048),
                        ],
                    ),
                ],
            ),
            FileEntry::directory(
                "tests",
                vec![FileEntry::file("integration_test.rs", 8192)],
            ),
        ],
    );

    println!("=== File System Tree ===\n");
    project.print_tree("", true);

    println!("\n=== Statistics ===\n");
    println!("Total size: {}", format_size(project.size()));
    println!("Total files: {}", project.count_files());
}
