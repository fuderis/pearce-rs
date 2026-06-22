pub mod addr;
pub use addr::Addr;

pub mod status;
pub use status::Status;

pub mod header;
pub use header::{Header, HeaderBody, Headers};

pub mod response;
pub use response::Response;

pub mod listener;
pub use listener::IpcListener;

pub use axum::{
    self,
    extract::{Json, Path as Paths, Query},
};
pub use urlencoding::{
    self, decode as url_decode, decode_binary as url_decode_binary, encode as url_encode,
    encode_binary as url_encode_binary,
};
pub use validator::{self, Validate, ValidationError};

use crate::prelude::*;
use axum::{
    Router,
    handler::Handler,
    routing::{get, post},
};
use tokio::net::TcpListener;

/// The Axum server wrapper
pub struct Server {
    router: Router,
}

impl Server {
    /// Creates a new Axum server
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Add any route (universal)
    pub fn route(mut self, path: &str, method_router: axum::routing::MethodRouter) -> Self {
        self.router = self.router.route(path, method_router);
        self
    }

    /// Add the POST-page handler
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, post(handler));
        self
    }

    /// Add the GET-page handler
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Launching the server at a specific address (TCP or UDS)
    #[async_recursion]
    pub async fn run(self, addr: impl Into<Addr> + Send + 'static) -> Result<()> {
        match addr.into() {
            // TCP protocol:
            Addr::Ip(socket_addr) => {
                let listener = TcpListener::bind(socket_addr).await?;
                axum::serve(listener, self.router).await?;
            }

            // IPC protocol (by socket name):
            Addr::Name(name) => {
                let path = if cfg!(unix) {
                    std::path::PathBuf::from(format!("/tmp/{name}.sock"))
                } else {
                    let base_dir =
                        std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
                    std::path::PathBuf::from(format!("{}\\..\\Local\\Temp\\{name}.sock", base_dir))
                };

                self.run(Addr::Path(path)).await?;
            }

            // IPC protocol (by socket path):
            Addr::Path(path) => {
                let listener = IpcListener::bind(&path)?;

                let _socket_guard = SocketGuard { path };
                let make_service = self.router.into_make_service();

                axum::serve(listener.inner, make_service).await?;
            }
        }

        Ok(())
    }
}

struct SocketGuard {
    path: std::path::PathBuf,
}

impl Drop for SocketGuard {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
