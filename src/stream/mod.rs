use crate::prelude::*;

pub use atoman::{Receiver, Sender};
pub use bytes::{self, Bytes, BytesMut};
pub use futures;

use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;

/// Creates the SSE stream body (Server-Sent Events)
pub fn stream_body<H, Fut>(handler: H) -> impl Stream<Item = Result<Bytes>>
where
    H: FnOnce(Sender<Bytes>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = channel::<Bytes>(None);

    tokio::spawn(async move {
        handler(tx).await;
    });

    // creating a stream that wraps each incoming chunk in the SSE format:
    futures::stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(Some(bytes)) => {
                // formatting as SSE: "data: <payload>\n\n"
                let mut sse_data = Vec::with_capacity(bytes.len() + 8);
                sse_data.extend_from_slice(b"data: ");
                sse_data.extend_from_slice(&bytes);
                sse_data.extend_from_slice(b"\n\n");

                Some((Ok(Bytes::from(sse_data)), rx))
            }
            Err(e) => Some((Err(e), rx)),
            Ok(None) => None,
        }
    })
}

/// Reads the SSE stream (Server-Sent Events)
pub fn stream_reader<T>(
    mut source: impl Stream<Item = Result<Bytes>> + Send + Unpin + 'static,
) -> Receiver<T>
where
    T: DeserializeOwned + Send + 'static,
{
    let (tx, rx) = channel::<T>(None);

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
                                        if tx.send(item).await.is_err() {
                                            return;
                                        }
                                    }
                                    Err(e) => {
                                        if tx.send_err(e.into()).await.is_err() {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tx.send_err(e.into()).await.ok();
                    return;
                }
            }
        }
    });

    rx
}
