//! Chapter 9: Creational Patterns - Builder Pattern

use std::marker::PhantomData;
use std::time::Duration;

// Standard Builder
#[derive(Debug, Clone)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout: Duration,
}

#[derive(Default)]
struct HttpRequestBuilder {
    method: Option<String>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout: Option<Duration>,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn build(self) -> Result<HttpRequest, &'static str> {
        Ok(HttpRequest {
            method: self.method.ok_or("method is required")?,
            url: self.url.ok_or("url is required")?,
            headers: self.headers,
            body: self.body,
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        })
    }
}

// Typestate Builder
mod typestate {
    use super::*;

    pub struct NoHost;
    pub struct HasHost;
    pub struct NoPort;
    pub struct HasPort;

    #[derive(Debug)]
    pub struct Connection {
        pub host: String,
        pub port: u16,
        pub use_tls: bool,
        pub pool_size: u32,
    }

    pub struct ConnectionBuilder<H, P> {
        host: Option<String>,
        port: Option<u16>,
        use_tls: bool,
        pool_size: u32,
        _host_state: PhantomData<H>,
        _port_state: PhantomData<P>,
    }

    impl ConnectionBuilder<NoHost, NoPort> {
        pub fn new() -> Self {
            Self {
                host: None,
                port: None,
                use_tls: false,
                pool_size: 10,
                _host_state: PhantomData,
                _port_state: PhantomData,
            }
        }
    }

    impl Default for ConnectionBuilder<NoHost, NoPort> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<P> ConnectionBuilder<NoHost, P> {
        pub fn host(self, host: &str) -> ConnectionBuilder<HasHost, P> {
            ConnectionBuilder {
                host: Some(host.to_string()),
                port: self.port,
                use_tls: self.use_tls,
                pool_size: self.pool_size,
                _host_state: PhantomData,
                _port_state: PhantomData,
            }
        }
    }

    impl<H> ConnectionBuilder<H, NoPort> {
        pub fn port(self, port: u16) -> ConnectionBuilder<H, HasPort> {
            ConnectionBuilder {
                host: self.host,
                port: Some(port),
                use_tls: self.use_tls,
                pool_size: self.pool_size,
                _host_state: PhantomData,
                _port_state: PhantomData,
            }
        }
    }

    impl<H, P> ConnectionBuilder<H, P> {
        pub fn use_tls(mut self, use_tls: bool) -> Self {
            self.use_tls = use_tls;
            self
        }

        pub fn pool_size(mut self, size: u32) -> Self {
            self.pool_size = size;
            self
        }
    }

    impl ConnectionBuilder<HasHost, HasPort> {
        pub fn build(self) -> Connection {
            Connection {
                host: self.host.unwrap(),
                port: self.port.unwrap(),
                use_tls: self.use_tls,
                pool_size: self.pool_size,
            }
        }
    }
}

fn main() {
    println!("=== Standard Builder Pattern ===\n");

    let get_request = HttpRequestBuilder::new()
        .method("GET")
        .url("https://api.example.com/users")
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build request");

    println!("GET request: {:?}", get_request);

    let invalid = HttpRequestBuilder::new().url("https://example.com").build();
    println!("Invalid request (no method): {:?}", invalid);

    println!("\n=== Typestate Builder Pattern ===\n");

    let connection = typestate::ConnectionBuilder::new()
        .host("localhost")
        .port(5432)
        .use_tls(true)
        .pool_size(20)
        .build();

    println!("Connection: {:?}", connection);

    // The following would NOT compile:
    // let invalid = typestate::ConnectionBuilder::new()
    //     .host("localhost")
    //     .build();  // Error: build() not available without port
}
