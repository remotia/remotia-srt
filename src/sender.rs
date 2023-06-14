use std::time::Instant;

use async_trait::async_trait;

use futures::SinkExt;

use remotia::{traits::{BorrowFrameProperties, FrameProcessor}, buffers::BytesMut};
use srt_tokio::SrtSocket;

pub struct SRTFrameSender<K> {
    buffer_key: K,
    socket: SrtSocket,
}

impl<K> SRTFrameSender<K> {
    pub fn new(buffer_key: K, socket: SrtSocket) -> Self {
        Self { buffer_key, socket }
    }
}

#[async_trait]
impl<F, K> FrameProcessor<F> for SRTFrameSender<K>
where
    K: Send,
    F: BorrowFrameProperties<K, BytesMut> + Send + 'static,
{
    async fn process(&mut self, frame_data: F) -> Option<F> {
        let buffer = frame_data
            .get_ref(&self.buffer_key)
            .unwrap()
            .clone()
            .freeze();

        self.socket.send((Instant::now(), buffer)).await.unwrap();

        Some(frame_data)
    }
}
