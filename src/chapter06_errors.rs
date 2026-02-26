//! Chapter 6: Error Handling Without Exceptions
//!
//! Custom error types, the ? operator, thiserror, and anyhow.

use std::fs;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },

    #[error("Invalid configuration format: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for {field}: {message}")]
    InvalidValue { field: String, message: String },

    #[error("IO error")]
    Io(#[from] io::Error),
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_seconds: u64,
}

fn parse_config(content: &str) -> Result<Config, ConfigError> {
    let mut host = None;
    let mut port = None;
    let mut max_connections = None;
    let mut timeout_seconds = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(ConfigError::InvalidFormat(format!(
                "Expected key=value, got: {}",
                line
            )));
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "host" => host = Some(value.to_string()),
            "port" => {
                port = Some(value.parse().map_err(|_| ConfigError::InvalidValue {
                    field: "port".to_string(),
                    message: format!("'{}' is not a valid port number", value),
                })?)
            }
            "max_connections" => {
                max_connections = Some(value.parse().map_err(|_| ConfigError::InvalidValue {
                    field: "max_connections".to_string(),
                    message: format!("'{}' is not a valid number", value),
                })?)
            }
            "timeout" => {
                timeout_seconds = Some(value.parse().map_err(|_| ConfigError::InvalidValue {
                    field: "timeout".to_string(),
                    message: format!("'{}' is not a valid number", value),
                })?)
            }
            _ => {}
        }
    }

    Ok(Config {
        host: host.ok_or(ConfigError::MissingField("host".to_string()))?,
        port: port.ok_or(ConfigError::MissingField("port".to_string()))?,
        max_connections: max_connections
            .ok_or(ConfigError::MissingField("max_connections".to_string()))?,
        timeout_seconds: timeout_seconds
            .ok_or(ConfigError::MissingField("timeout".to_string()))?,
    })
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            ConfigError::FileNotFound {
                path: path.to_string(),
            }
        } else {
            ConfigError::Io(e)
        }
    })?;

    parse_config(&content)
}

fn run_application() -> anyhow::Result<()> {
    use anyhow::Context;

    let config_content = r#"
        host = localhost
        port = 8080
        max_connections = 100
        timeout = 30
    "#;

    let config = parse_config(config_content).context("Failed to parse configuration")?;
    println!("Loaded configuration: {:?}", config);

    validate_config(&config).context("Configuration validation failed")?;
    println!("Configuration is valid!");
    Ok(())
}

fn validate_config(config: &Config) -> anyhow::Result<()> {
    use anyhow::bail;

    if config.port == 0 {
        bail!("Port cannot be zero");
    }
    if config.max_connections == 0 {
        bail!("max_connections must be greater than zero");
    }
    if config.timeout_seconds == 0 {
        bail!("timeout must be greater than zero");
    }
    Ok(())
}

fn main() {
    println!("=== Error Handling Patterns ===\n");

    let result = parse_config("host = localhost\nport = 8080");
    match result {
        Ok(config) => println!("Config: {:?}", config),
        Err(e) => println!("Error (expected): {}", e),
    }

    println!("\n=== Loading Config from File ===\n");
    match load_config("/nonexistent/config.txt") {
        Ok(config) => println!("Loaded: {:?}", config),
        Err(e) => println!("Error loading config: {}", e),
    }

    println!("\n=== Running Application ===\n");
    if let Err(e) = run_application() {
        println!("Application error: {}", e);
    }
}
