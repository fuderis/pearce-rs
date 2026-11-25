use super::ServerError;
use crate::{
    prelude::*,
    server::{Json, Paths, Response},
};

use dashmap::DashMap;
use std::{sync::LazyLock, time::Duration};
use tokio::{sync::oneshot, time};

static CALLBACKS: LazyLock<DashMap<String, oneshot::Sender<JsonValue>>> =
    LazyLock::new(DashMap::new);

pub struct Callback;

impl Callback {
    /// Registers callback with specific event `id`.
    pub fn register(id: impl Into<String>) -> CallbackReceiver {
        let id = id.into();
        let (tx, rx) = oneshot::channel();
        CALLBACKS.insert(id.clone(), tx);
        CallbackReceiver { id, rx }
    }

    /// Removes callback by ID.
    pub fn remove(id: &str) -> bool {
        CALLBACKS.remove(id).is_some()
    }
}

/// Hidden `POST` handler `/callback/{id}`.
pub(crate) async fn handle_callback(
    Paths(id): Paths<String>,
    Json(payload): Json<JsonValue>,
) -> Response {
    if let Some((_, tx)) = CALLBACKS.remove(&id) {
        let _ = tx.send(payload);
    }

    Response::ok()
}

/// Server callback receiver (removes callback on drop).
pub struct CallbackReceiver {
    id: String,
    rx: oneshot::Receiver<JsonValue>,
}

impl CallbackReceiver {
    /// Receives callback output.
    /// Returns `Ok(None)` if the sender was dropped without sending a value,
    /// or `Err(ServerError::Timeout)` when the timeout expires.
    pub async fn recv<T>(&mut self, timeout: Duration) -> Result<Option<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let wait_res = time::timeout(timeout, &mut self.rx).await;

        // guaranteed cleanup in case of timeout, error, or success
        CALLBACKS.remove(&self.id);

        match wait_res {
            Ok(Ok(value)) => Ok(Some(json::from_value(value)?)),
            Ok(Err(_)) => Ok(None),
            Err(_) => Err(ServerError::Timeout.into()),
        }
    }
}

impl Drop for CallbackReceiver {
    fn drop(&mut self) {
        CALLBACKS.remove(&self.id);
    }
}
