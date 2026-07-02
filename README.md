[![github]](https://github.com/fuderis/pearce-rs)&ensp;
[![crates-io]](https://crates.io/crates/pearce)&ensp;
[![docs-rs]](https://docs.rs/pearce)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Pearce: A Friendly Web Framework based on Axum

Pearce is a high-level, asynchronous web-framework built on top of [Axum](https://docs.rs/axum). It is designed to provide a clean, declarative API for building robust microservices,
focusing on reducing boilerplate through "smart defaults" and internal automation.

## Features:

* **Smart Response Builders**: Methods like `.json()`, `.html()`, and `.stream()` automatically set the correct `Content-Type` and required headers.
* **Automatic SSE Configuration**: Streaming responses come pre-configured with `no-cache`, `keep-alive`, and `nosniff` headers.
* **Safe File Downloads**: Built-in attachment support with automatic `URL-encoding` for `non-ASCII` filenames.
* **Declarative Validation**: Integration with the `Validate` trait for complex request-level schema logic.
* **Simplified Headers**: Easy-to-use `Headers` extractor to avoid manual `HeaderMap` manipulation.
* **Flexible Payloads**: Built-in support for `JSON` data parsing.

## Examples:

### Server [feature `server`]:
```rust
use pearce::{Headers, Json, Query, Response, Server, Stream, Validate, ValidationError};

use serde::{Deserialize, Serialize};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Server::new()
        .get("/", home_page)
        .get("/hello", hello_page)
        .post("/test", handle_test)
        .post("/wait", handle_waiter)
        .post("/auth", handle_auth)
        .run(8080) // or .run(([127,0,0,1], 8080)) / .run("test-socket") / .run(PathBuf::from("/tmp/test.sock"))
        .await
}
```

#### Basic Handlers

Pearce handles routing and payload extraction automatically. Use Query<T> for URL parameters and Json<T> for request bodies.

```rust
#[derive(Deserialize)]
struct QueryData {
    #[serde(default)]
    name: Option<String>,
}

// API: Standard text response
async fn home_page() -> Response {
    Response::ok().text("Hello, World!")
}

// API: Using Query parameters
async fn hello_page(payload: Query<QueryData>) -> Response {
    let name = payload.name.as_deref().unwrap_or("World");
    Response::ok().text(format!("Hello, {name}!"))
}
```

#### Real-Time Streaming (SSE) [feature `stream`]

Create asynchronous streams for AI responses or progress updates with Stream::body.

```rust
async fn handle_waiter() -> Response {
    Response::ok().stream(async move |tx| {
        for i in (1..=5).rev() {
            time::sleep(time::Duration::from_secs(1)).await;

            let msg = format!("Please, wait {i} seconds...");
            println!("{msg}");

            tx.send(msg).await.ok();
        }

        let msg = "Finished!";
        println!("{msg}");

        tx.send(msg).await.ok();
    })
}
```

#### Headers & Validation

Pearce makes it easy to secure your API by extracting headers and validating incoming data structures using the Validate trait.

```rust
#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "Self::validate_identity"))]
struct LoginData {
    #[validate(email(message = "Invalid E-Mail address"))]
    email: Option<String>,
    login: Option<String>,
    #[validate(length(min = 8, message = "Password too short"))]
    password: String,
}

impl LoginData {
    fn validate_identity(data: &LoginData) -> Result<(), ValidationError> {
        if data.email.is_none() && data.login.is_none() {
            return Err(ValidationError::new("missing_identity")
                .with_message("Provide either email or login".into()));
        }
        Ok(())
    }
}

async fn handle_auth(headers: Headers, payload: Json<LoginData>) -> Response {
    // easy header access
    let user_agent = headers.get("user-agent");  // or .get(Header::UserAgent)
    if user_agent.trim().is_empty() || user_agent.contains("Bot") {
        return Response::forbidden().text("No bots allowed.");  // or ::new(403) or ::new(Status::Forbidden).
    }

    // data validation
    if let Err(e) = payload.validate() {
        return Response::bad_entity().json(&format!("Validation failed: {e:?}"));
    }

    let identity = payload.email.as_ref().or(payload.login.as_ref()).unwrap();
    Response::ok().text(format!("Welcome, {identity}!"))
}
```

### Client [feature `client`]

Pearce includes a pre-configured Client for both classic TCP networking and zero-overhead Inter-Process Communication
(via Unix Domain Sockets on Linux/macOS and AF_UNIX on Windows wrappers).

```rust
use pearce::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // standard TCP client
    let tcp_client = Client::tcp();
    let response = tcp_client.get("https://example.com").send().await;

    // cross-platform IPC client (Unix Domain Sockets)
    let ipc_client = Client::ipc("/tmp/test.sock");
    let payload = serde_json::json!({ "login": "admin", "password": "..." });
    let response = ipc_client.post("/auth").json(&payload).send().await;

    Ok(())
}
```

#### SSE Stream Reading [feature `stream`]
```rust
use pearce::{Client, StreamExt};

#[derive(Debug, serde::Deserialize)]
struct Event {
    id: u64,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // connect to an SSE endpoint
    let mut rx = Client::tcp()
        .get("http://localhost:8080/events")
        .stream::<Event>()
        .await?;

    // read incoming events
    while let Some(event) = rx.recv().await? {
        println!("Received event: {:?}", event);
    }

    Ok(())
}
```

## License & Feedback:

> Distributed under the [MIT](https://github.com/fuderis/pearce-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
