use std::time::Duration;

use async_trait::async_trait;
use bytes::BytesMut;
use remotia::traits::{BorrowableFrameProperties, FrameProcessor};

use futures::TryStreamExt;
use srt_tokio::SrtSocket;

pub struct SRTFrameReceiver<K> {
    buffer_key: K,
    socket: SrtSocket,
}

impl<K> SRTFrameReceiver<K> {
    pub async fn new(buffer_key: K, socket: SrtSocket) -> Self {
        Self { buffer_key, socket }
    }
}


#[async_trait]
impl<F, K> FrameProcessor<F> for SRTFrameReceiver<K>
where
    K: Send,
    F: BorrowableFrameProperties<K, BytesMut> + Send + 'static,
{
    async fn process(&mut self, mut frame_data: F) -> Option<F> {
        let (_, received_buffer) = match self.socket.try_next().await {
            Ok(result) => result.unwrap(),
            Err(err) => {
                log::error!("Reception error: {}", err);
                return None;
            }
        };

        frame_data
            .get_mut_ref(&self.buffer_key)
            .unwrap()
            .copy_from_slice(&received_buffer);

        Some(frame_data)
    }
}
