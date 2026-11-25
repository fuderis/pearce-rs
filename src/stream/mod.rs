//! Server-Sent Events stream module.

use crate::prelude::*;

pub use bytes::{self, Bytes, BytesMut};
pub use futures;

pub use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc;

pub type SseSender<T = Bytes> = mpsc::Sender<Result<T, DynError>>;
pub type SseReceiver<T> = mpsc::Receiver<Result<T, DynError>>;

/// Creates SSE stream body (Server-Sent Events).
pub fn stream_body<H, Fut>(
    handler: H,
    buffer_size: usize,
) -> impl Stream<Item = Result<Bytes, DynError>>
where
    H: FnOnce(mpsc::Sender<Result<Bytes, DynError>>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<Result<Bytes, DynError>>(buffer_size);

    tokio::spawn(async move {
        handler(tx).await;
    });

    // creating a stream that wraps each incoming chunk in the SSE format
    futures::stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Some(Ok(bytes)) => {
                // formatting as SSE: "data: <payload>\n\n"
                let mut sse_data = Vec::with_capacity(bytes.len() + 8);
                sse_data.extend_from_slice(b"data: ");
                sse_data.extend_from_slice(&bytes);
                sse_data.extend_from_slice(b"\n\n");

                Some((Ok(Bytes::from(sse_data)), rx))
            }
            Some(Err(e)) => Some((Err(e), rx)),
            None => None,
        }
    })
}

/// Reads SSE stream for StreamExt implementations (Server-Sent Events).
pub fn stream_reader<T>(
    mut source: impl Stream<Item = Result<Bytes, DynError>> + Send + Unpin + 'static,
    buffer_capacity: usize,
) -> mpsc::Receiver<Result<T, DynError>>
where
    T: DeserializeOwned + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<Result<T, DynError>>(buffer_capacity);

    tokio::spawn(async move {
        let mut buffer = BytesMut::new();

        while let Some(res) = source.next().await {
            match res {
                Ok(bytes) => {
                    // insert new bytes
                    buffer.extend_from_slice(&bytes);

                    // search chunk splitter (\n\n)
                    while let Some(pos) = buffer.windows(2).position(|w| w == b"\n\n") {
                        // cut off exactly to the end of the message in O(1) without copying
                        // (buffer keeps everything AFTER pos +2, and full_message gets the start)
                        let full_message = buffer.split_to(pos + 2).freeze();

                        // parsing from the byte slice (from_utf8 does not allocate)
                        if let Ok(line) = std::str::from_utf8(&full_message) {
                            let trimmed = line.trim();

                            if trimmed.starts_with("data:") {
                                let json_part = trimmed[5..].trim();

                                if json_part.is_empty() {
                                    continue;
                                }

                                if json_part == "[DONE]" {
                                    return;
                                }

                                match serde_json::from_str::<T>(json_part) {
                                    Ok(item) => {
                                        if tx.send(Ok(item)).await.is_err() {
                                            return;
                                        }
                                    }
                                    Err(e) => {
                                        if tx.send(Err(e.into())).await.is_err() {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    return;
                }
            }
        }
    });

    rx
}
