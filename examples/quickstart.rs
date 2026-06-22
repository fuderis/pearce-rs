#![cfg(feature = "server")]
use pearce::{Headers, Json, Query, Response, Server, Stream, Validate, ValidationError};
use serde::{Deserialize, Serialize};
use tokio::time;

use std::result::Result as StdResult;
pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = StdResult<T, DynError>;

// The query data
#[derive(Debug, Deserialize)]
struct QueryData {
    #[serde(default)]
    name: Option<String>,
}

impl QueryData {
    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("World")
    }
}

// The response data
#[derive(Debug, Serialize)]
struct ResponseData {
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    Server::new()
        .get("/", home_page)
        .get("/hello", hello_page)
        .post("/test", handle_test)
        .post("/wait", handle_waiter)
        .post("/auth", handle_auth)
        .run(8080)
        .await
}

// --- SIMPLE HANDLERS ---

// API: home page
async fn home_page() -> Response {
    Response::ok().text("Hello, World!")
}

// API: prints "hello world"
async fn hello_page(payload: Query<QueryData>) -> Response {
    Response::ok().text(format!("Hello, {}!", payload.get_name()))
}

// API: prints "hello world" (JSON)
async fn handle_test(payload: Json<QueryData>) -> Response {
    Response::ok().json(&ResponseData {
        message: format!("Hello, {}!", payload.get_name()),
    })
}

// --- EVENT-STREAM ---

// API: returns event-stream
async fn handle_waiter() -> Response {
    let body = Stream::body(async move |tx| {
        for i in (1..=5).rev() {
            time::sleep(time::Duration::from_secs(1)).await;

            let msg = format!("Please, wait {i} seconds...");
            println!("{msg}");

            tx.send(msg).ok();
        }

        let msg = "Finished!";
        println!("{msg}");

        tx.send(msg).ok();
    });

    Response::ok().stream(body)
}

// --- HEADERS & VALIDATION ---

// The query data
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
    fn validate_identity(data: &LoginData) -> StdResult<(), ValidationError> {
        if data.email.is_none() && data.login.is_none() {
            return Err(ValidationError::new("missing_identity")
                .with_message("Provide either email or login".into()));
        }

        Ok(())
    }
}

// API: authorizes the user
async fn handle_auth(headers: Headers, payload: Json<LoginData>) -> Response {
    // check user-agent header:
    let user_agent = headers.get("user-agent").unwrap_or("");
    if user_agent.trim().is_empty() || user_agent.contains("Bot") || user_agent.contains("python") {
        return Response::new(403).text("Access denied: automated requests are not allowed.");
    }

    // validate login data:
    if let Err(e) = payload.validate() {
        return Response::new(422).json(&format!("Validation failed: {:?}", e));
    }
    let identity = payload.email.as_ref().or(payload.login.as_ref()).unwrap();

    Response::ok().text(format!("Welcome, {}! Auth successful.", identity))
}
