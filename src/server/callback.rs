use crate::{Json, Paths, Response, prelude::*};

pub static CALLBACKS: SharedMap<String, Sender<serde_json::Value>> = SharedMap::new();

/// Waits for callback with specific event `id`.
pub async fn wait_callback<T>(id: impl Into<String>, timeout: Duration) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let id_str = id.into();
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let sender = Sender::Bounded(tx);

    // register channel in the global state
    CALLBACKS.insert(id_str.clone(), sender).await;

    // waiting for data from the client with a timeout
    let mut receiver = Receiver::Bounded(rx);
    let wait_res = tokio::time::timeout(timeout, receiver.recv()).await;

    // guaranteed memory clearing (even if a timeout occurs)
    CALLBACKS.remove(&id_str).await;

    match wait_res {
        Ok(Ok(Some(value))) => {
            let parsed = serde_json::from_value(value)?;
            Ok(parsed)
        }
        Ok(Ok(None)) => Err(Error::CallbackClosed.into()),
        Ok(Err(err)) => Err(err),
        Err(_) => Err(Error::CallbackTimeout.into()),
    }
}

/// Hidden `POST` handler `/callback/:id`.
pub(crate) async fn handle_callback(
    Paths(id): Paths<String>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    // look for and immediately REMOVE the waiting channel
    if let Some(item) = CALLBACKS.remove(&id).await {
        let guard = item.read().await;
        // send payload to the waiting stream
        let _ = guard.send_async(payload).await;
    }

    // instantly return 200 OK to the client
    Response::ok()
}
