pub mod addr;
pub use addr::Addr;

pub mod status;
pub use status::Status;

pub mod header;
pub use header::{Header, HeaderBody, Headers};

pub mod response;
pub use response::Response;

use crate::prelude::*;
use axum::{
    Router,
    handler::Handler,
    routing::{get, post},
};
use tokio::fs;
use tokio::net::{TcpListener, UnixListener};

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
    pub async fn run(self, addr: impl Into<Addr>) -> Result<()> {
        match addr.into() {
            // TCP protocol:
            Addr::Ip(socket_addr) => {
                let listener = TcpListener::bind(socket_addr).await?;
                axum::serve(listener, self.router).await?;
            }

            // UDS protocol:
            Addr::Path(path_buf) => {
                // remove old socket (is exists):
                if path_buf.exists() {
                    fs::remove_file(&path_buf).await?;
                }

                let listener = UnixListener::bind(&path_buf)?;
                let make_service = self.router.into_make_service();
                axum::serve(listener, make_service).await?;
            }
        }

        Ok(())
    }
}
