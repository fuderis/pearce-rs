//! HTTP server module.

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

pub mod callback;
pub use callback::wait_callback;

pub use axum::{
    self,
    extract::{Json, Path as Paths, Query},
    routing,
};
pub use urlencoding::{
    self, decode as url_decode, decode_binary as url_decode_binary, encode as url_encode,
    encode_binary as url_encode_binary,
};
pub use validator::{self, Validate, ValidationError};

use crate::prelude::*;
use axum::{Router, handler::Handler};
use tokio::net::TcpListener;

/// HTTP server (based on [axum]).
pub struct Server {
    router: Router,
    enable_callback: bool,
}

impl Server {
    /// Creates new server.
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            enable_callback: false,
        }
    }

    /// Add any route (universal).
    pub fn route(mut self, path: &str, method_router: axum::routing::MethodRouter) -> Self {
        self.router = self.router.route(path, method_router);
        self
    }

    /// Adds `POST` endpoint handler.
    pub fn post<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::post(handler))
    }

    /// Adds `GET` endpoint handler.
    pub fn get<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::get(handler))
    }

    /// Adds `DELETE` endpoint handler.
    pub fn delete<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::delete(handler))
    }

    /// Adds `PUT` endpoint handler.
    pub fn put<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::put(handler))
    }

    /// Adds `PATCH` endpoint handler.
    pub fn patch<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::patch(handler))
    }

    /// Adds `HEAD` endpoint handler.
    pub fn head<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::head(handler))
    }

    /// Adds `OPTIONS` endpoint handler.
    pub fn options<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.route(path, routing::options(handler))
    }

    /// Includes hidden POST endpoint `/callback/:id`.
    pub fn callback(mut self, enable: bool) -> Self {
        self.enable_callback = enable;
        self
    }

    /// Launching server at specific address (TCP/IPC).
    #[async_recursion]
    pub async fn run(mut self, addr: impl Into<Addr> + Send + 'static) -> Result<()> {
        if self.enable_callback {
            self.router = self
                .router
                .route("/callback/:id", routing::post(callback::handle_callback));
        }

        match addr.into() {
            // TCP protocol
            Addr::Ip(socket_addr) => {
                let listener = TcpListener::bind(socket_addr).await?;
                axum::serve(listener, self.router).await?;
            }

            // IPC protocol (by socket name)
            Addr::Name(name) => {
                self.run(Addr::Path(path!("$temp/{name}.sock"))).await?;
            }

            // IPC protocol (by socket path)
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
