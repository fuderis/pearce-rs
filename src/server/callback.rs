use crate::{
    prelude::*,
    server::{Json, Paths, Response},
};

use tokio::time;

pub static CALLBACKS: SharedMap<String, Callback> = SharedMap::new();

/// Server callback sender.
#[derive(Clone)]
pub struct Callback {
    tx: Sender<JsonValue>,
}

impl Callback {
    /// Registers callback with specific event `id`.
    pub async fn register(id: impl Into<String>) -> CallbackReceiver {
        let id = id.into();

        // register in the global state
        let (tx, rx) = atoman::bounded_channel(1);
        CALLBACKS.insert(id.clone(), Self { tx }).await;

        CallbackReceiver { id, rx }
    }

    /// Removes callback by ID.
    pub async fn remove(id: impl Into<String>) -> Option<SharedItem<Callback>> {
        CALLBACKS.remove(&id.into()).await
    }
}

/// Hidden `POST` handler `/callback/{id}`.
pub(crate) async fn handle_callback(
    Paths(id): Paths<String>,
    Json(payload): Json<JsonValue>,
) -> Response {
    // look for and immediately REMOVE the waiting channel
    if let Some(item) = CALLBACKS.remove(&id).await {
        let guard = item.read().await;
        // send payload to the waiting stream
        let _ = guard.tx.send_async(payload).await;
    }

    // instantly return 200 OK to the client
    Response::ok()
}

/// Server callback receiver (removes callback on drop).
pub struct CallbackReceiver {
    id: String,
    rx: Receiver<JsonValue>,
}

impl CallbackReceiver {
    /// Receives callback output (returns None if timeout).
    pub async fn recv<T>(&mut self, timeout: Duration) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        // waiting for data from the client with a timeout
        let wait_res = time::timeout(timeout, self.rx.recv()).await;

        // guaranteed memory clearing (even if a timeout occurs)
        CALLBACKS.remove(&self.id).await;

        match wait_res {
            Ok(Ok(opt)) => {
                if let Some(value) = opt {
                    Ok(Some(json::from_value(value)?))
                } else {
                    Ok(None)
                }
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Ok(None),
        }
    }
}

impl Drop for CallbackReceiver {
    fn drop(&mut self) {
        let id = self.id.clone();
        tokio::spawn(async move {
            CALLBACKS.remove(&id).await;
        });
    }
}
